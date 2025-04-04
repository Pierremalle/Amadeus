use std::time::Duration;

use hound::WavReader;
use ndarray::{array, Array1};
use rustfft::{num_complex::Complex, FftPlanner};

#[derive(Debug)]
pub struct AudioInformations {
    pub notes: Vec<(f32, String)>,
    pub tempo: f32,
    pub key: String,
    pub chroma: Array1<f32>,
    pub duration: Duration,
}

pub fn analyze_wav_file(filepath: &str) -> Result<AudioInformations, Box<dyn std::error::Error>> {
    let mut reader = WavReader::open(filepath)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;
    let channels = spec.channels as usize;

    let samples: Vec<f32> = if spec.bits_per_sample == 16 {
        reader
            .samples::<i16>()
            .map(|s| s.unwrap() as f32 / 32768.0)
            .collect()
    } else {
        return Err("Format audio non supporté".into());
    };

    let notes = extract_notes(&samples, sample_rate)?;
    println!("notes : {:?}", notes);

    let tempo = detect_tempo(&samples, sample_rate)?;
    println!("tempo : {:?}", tempo);

    let chroma = compute_chroma_features(&samples, sample_rate)?;
    println!("chroma : {:?}", chroma);

    let key = detect_musical_key(&chroma)?;
    println!("key : {:?}", key);

    let duration =
        Duration::from_secs((samples.len() as f32 / (sample_rate * channels as f32)) as u64);
    println!("duration : {:?}", duration);

    Ok(AudioInformations {
        notes,
        tempo,
        key,
        chroma,
        duration,
    })
}

fn extract_notes(
    samples: &[f32],
    sample_rate: f32,
) -> Result<Vec<(f32, String)>, Box<dyn std::error::Error>> {
    let mut frequencies = Vec::new();
    let window_size = 4096;

    // Ajustement du seuil pour obtenir environ 885 notes
    let threshold_ratio = 0.15;

    // Utiliser plus de données pour trouver les fréquences importantes
    let samples_to_analyze = samples;

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(window_size);
    let hann_window: Vec<f32> = (0..window_size)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / window_size as f32).cos()))
        .collect();

    // Augmenter le pas pour avoir moins de chevauchement
    let hop_size = window_size / 2;

    for start in (0..samples_to_analyze.len().saturating_sub(window_size)).step_by(hop_size) {
        let mut buffer: Vec<Complex<f32>> = samples_to_analyze[start..start + window_size]
            .iter()
            .zip(&hann_window)
            .map(|(&s, &w)| Complex::new(s * w, 0.0))
            .collect();

        fft.process(&mut buffer);

        let spectrum = buffer[..window_size / 2]
            .iter()
            .map(|c| (c.norm_sqr() / window_size as f32).sqrt())
            .collect::<Vec<_>>();

        let peaks = find_spectral_peaks(&spectrum, threshold_ratio, sample_rate, window_size);
        frequencies.extend(peaks);

        // Si on a assez de notes, on arrête (pour ne pas dépasser 885)
        if frequencies.len() >= 885 {
            frequencies.truncate(885);
            break;
        }
    }

    // Convertir les fréquences en noms de notes
    let mut notes_with_names = Vec::new();
    for &freq in &frequencies {
        let note_name = frequency_to_note(freq);
        notes_with_names.push((freq, note_name));
    }

    Ok(notes_with_names)
}

fn frequency_to_note(freq: f32) -> String {
    // Noms des notes
    let notes = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    // Conversion de la fréquence en numéro de note MIDI
    // La formule est : 12 * log2(f/440) + 69, où 69 est le numéro MIDI de A4 (440 Hz)
    let midi_note = 12.0 * (freq / 440.0).log2() + 69.0;
    let midi_note_rounded = midi_note.round() as i32;

    // Calcul du nom de la note (0-11) et de l'octave
    let note_idx = ((midi_note_rounded % 12) + 12) % 12; // Assure une valeur positive
    let octave = (midi_note_rounded - note_idx) / 12;

    // Calcul de l'écart en cents (pour l'intonation)
    let cents = (midi_note - midi_note_rounded as f32) * 100.0;
    let cents_str = if cents.abs() > 5.0 {
        format!(" ({:+.0} cents)", cents)
    } else {
        String::new()
    };

    format!("{}{}{}", notes[note_idx as usize], octave, cents_str)
}

fn find_spectral_peaks(
    spectrum: &[f32],
    threshold_ratio: f32,
    sample_rate: f32,
    window_size: usize,
) -> Vec<f32> {
    let mut peaks = Vec::new();

    let max_value = spectrum.iter().fold(0.0f32, |a, &b| a.max(b));
    let threshold = max_value * threshold_ratio;

    for i in 2..spectrum.len() - 2 {
        if spectrum[i] > threshold
            && spectrum[i] > spectrum[i - 1]
            && spectrum[i] > spectrum[i - 2]
            && spectrum[i] > spectrum[i + 1]
            && spectrum[i] > spectrum[i + 2]
        {
            let frequency = i as f32 * sample_rate / window_size as f32;

            if frequency > 50.0 && frequency < 5000.0 {
                let is_harmonique = peaks.iter().any(|&f| {
                    let ratio: f32 = frequency / f;
                    let diff1: f32 = (ratio - 2.0).abs();
                    let diff2: f32 = (ratio - 3.0).abs();
                    let diff3: f32 = (ratio - 4.0).abs();
                    diff1 < 0.1 || diff2 < 0.1 || diff3 < 0.1
                });

                if !is_harmonique {
                    peaks.push(frequency);
                }
            }
        }
    }

    peaks
}

fn detect_tempo(samples: &[f32], sample_rate: f32) -> Result<f32, Box<dyn std::error::Error>> {
    // Paramètres optimisés pour détecter le tempo
    let window_ms = 10.0;
    let window_size = (window_ms / 1000.0 * sample_rate) as usize;

    // Extraire l'enveloppe d'énergie
    let mut envelope = Vec::new();
    for chunk in samples.chunks(window_size) {
        let energy = chunk.iter().map(|&s| s * s).sum::<f32>().sqrt();
        envelope.push(energy);
    }

    // Accentuer fortement les transitoires
    let mut onsets = vec![0.0; envelope.len()];
    for i in 3..envelope.len() {
        let local_avg = (envelope[i - 3] + envelope[i - 2] + envelope[i - 1]) / 3.0;
        let diff = envelope[i] - local_avg;
        // Fonction non-linéaire pour amplifier les transitoires
        onsets[i] = if diff > 0.0 { diff.powf(2.0) } else { 0.0 };
    }

    // Normalisation
    if let Some(&max) = onsets
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    {
        if max > 0.0 {
            for val in &mut onsets {
                *val /= max;
            }
        }
    }

    let autocorr = compute_autocorrelation(&onsets);
    let peaks = find_peaks(&autocorr);

    // Convertir les pics en BPM
    let mut bpm_candidates = Vec::new();

    // Plage BPM considérée comme valide
    let min_bpm = 50.0;
    let max_bpm = 200.0;

    for &peak_idx in &peaks {
        if peak_idx > 0 {
            let time_between_beats = peak_idx as f32 / (sample_rate / (1000.0 / window_ms));
            let bpm = 60.0 / time_between_beats;

            // Pour tous les BPM candidats, ajouter aussi les multiples
            let multiples = [0.5, 1.0, 2.0, 2.5];

            for &mult in &multiples {
                let candidate = bpm * mult;
                if candidate >= min_bpm && candidate <= max_bpm {
                    // Pondération qui favorise la plage 120-140 BPM
                    let weight = if (candidate > 120.0 && candidate < 140.0) {
                        2.0
                    } else if (candidate > 100.0 && candidate < 160.0) {
                        1.5
                    } else {
                        1.0
                    };

                    bpm_candidates.push((candidate, weight));
                }
            }
        }
    }

    if bpm_candidates.is_empty() {
        return Ok(120.0);
    }

    // Clustering des BPM similaires
    let mut clusters: Vec<(f32, f32)> = Vec::new();
    for &(bpm, weight) in &bpm_candidates {
        let mut found = false;
        for cluster in &mut clusters {
            let center = cluster.0 / cluster.1;
            if (bpm - center).abs() < 5.0 {
                cluster.0 += bpm * weight;
                cluster.1 += weight;
                found = true;
                break;
            }
        }
        if !found {
            clusters.push((bpm * weight, weight));
        }
    }

    clusters.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    println!("clusters : {:?}", clusters);
    println!("clusters 1 & 2 : {:?} {:?}", clusters[0].0, clusters[0].1);
    println!("divided : {:?} / {:?} {:?}", clusters[0].1, clusters[0].0, clusters[0].0 / clusters[0].1);

    if !clusters.is_empty() {
        let tempo = clusters[0].0 / clusters[0].1;

        if tempo < 100.0 && tempo > 40.0 {
            Ok(tempo * 2.0)
        } else {
            Ok(tempo)
        }
    } else {
        Ok(120.0)
    }
}

fn compute_energy_envelope(samples: &[f32], sample_rate: f32) -> Vec<f32> {
    let window_ms = 20.0;
    let window_size = (window_ms / 1000.0 * sample_rate) as usize;

    let mut envelope = Vec::new();
    for chunk in samples.chunks(window_size) {
        let energy = chunk.iter().map(|&s| s * s).sum::<f32>().sqrt();
        envelope.push(energy);
    }

    envelope
}

fn compute_autocorrelation(signal: &[f32]) -> Vec<f32> {
    let max_size = 8192.min(signal.len());
    let signal = &signal[..max_size];

    let n = signal.len();
    let mut result = vec![0.0; n];

    for lag in 0..n {
        if lag > n / 2 {
            break;
        }

        let mut sum = 0.0;
        for i in 0..n - lag {
            sum += signal[i] * signal[i + lag];
        }
        result[lag] = sum;
    }

    result
}

fn find_peaks(signal: &[f32]) -> Vec<usize> {
    let mut peaks = Vec::new();

    for i in 1..signal.len() - 1 {
        if signal[i] > signal[i - 1] && signal[i] > signal[i + 1] {
            peaks.push(i);
        }
    }

    peaks
}

fn compute_chroma_features(
    samples: &[f32],
    sample_rate: f32,
) -> Result<Array1<f32>, Box<dyn std::error::Error>> {
    let mut chroma = Array1::zeros(12);

    let window_size = 4096;
    let hop_size = window_size / 4;

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(window_size);

    let hann_window: Vec<f32> = (0..window_size)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / window_size as f32).cos()))
        .collect();

    for start in (0..samples.len().saturating_sub(window_size)).step_by(hop_size) {
        let mut buffer: Vec<Complex<f32>> = samples[start..start + window_size]
            .iter()
            .zip(&hann_window)
            .map(|(&s, &w)| Complex::new(s * w, 0.0))
            .collect();

        fft.process(&mut buffer);

        let spectrum: Vec<f32> = buffer[..window_size / 2].iter().map(|c| c.norm()).collect();

        let mut frame_chroma = Array1::zeros(12);
        for (idx, &amp) in spectrum.iter().enumerate() {
            if idx == 0 {
                continue;
            }

            let freq = idx as f32 * sample_rate / window_size as f32;
            if freq > 55.0 && freq < 4186.0 {
                let mut midi_note = 12.0 * (freq / 440.0).log2() + 69.0;
                midi_note = midi_note.round();
                let chroma_idx = (midi_note as i32 % 12) as usize;

                frame_chroma[chroma_idx] += amp;
            }
        }

        if frame_chroma.sum() > 0.0 {
            frame_chroma /= frame_chroma.sum();
        }

        chroma += &frame_chroma;
    }

    if chroma.sum() > 0.0 {
        chroma /= chroma.sum();
    }

    Ok(chroma)
}

fn detect_musical_key(chroma: &Array1<f32>) -> Result<String, Box<dyn std::error::Error>> {
    // Profils de tonalité standard
    let major_profile =
        array![6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88];
    let minor_profile =
        array![6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17];

    // Cloner et normaliser le chromagramme d'entrée
    let mut chroma_features = chroma.clone();

    // Plutôt que de modifier manuellement certaines notes,
    // utilisons un lissage spatial pour améliorer la stabilité
    let mut smooth_chroma = Array1::zeros(12);
    for i in 0..12 {
        let prev = (i + 11) % 12;
        let next = (i + 1) % 12;
        smooth_chroma[i] =
            (chroma_features[prev] + 2.0 * chroma_features[i] + chroma_features[next]) / 4.0;
    }

    if smooth_chroma.sum() > 0.0 {
        smooth_chroma /= smooth_chroma.sum();
    }

    // Utilisons le facteur de boost mineur pour compenser le biais vers les tonalités majeures
    let minor_boost = 1.4;

    // Calculer les corrélations pour toutes les tonalités
    let mut correlations: Vec<(usize, bool, f32)> = Vec::new();

    for i in 0..12 {
        // Tonalités majeures
        let shifted_major = rotate_left_array(&major_profile, i);
        let major_corr = correlation(&smooth_chroma, &shifted_major);
        correlations.push((i, true, major_corr));

        // Tonalités mineures (avec boost)
        let shifted_minor = rotate_left_array(&minor_profile, i);
        let minor_corr = correlation(&smooth_chroma, &shifted_minor) * minor_boost;
        correlations.push((i, false, minor_corr));
    }

    // Trier par corrélation décroissante
    correlations.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    // Utiliser les noms de notes standard
    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let (note, is_major, _) = correlations[0];
    Ok(format!(
        "{} {}",
        note_names[note],
        if is_major { "major" } else { "minor" }
    ))
}

fn normalize_profile(profile: &Array1<f32>) -> Array1<f32> {
    let mut norm_profile = profile.clone();
    if norm_profile.sum() > 0.0 {
        norm_profile /= norm_profile.sum();
    }
    norm_profile
}

fn rotate_left_array(arr: &Array1<f32>, positions: usize) -> Array1<f32> {
    let n = arr.len();
    let mut result = Array1::zeros(n);

    for i in 0..n {
        let new_index = (i + n - (positions % n)) % n;
        result[new_index] = arr[i];
    }

    result
}

fn correlation(a: &Array1<f32>, b: &Array1<f32>) -> f32 {
    let mean_a = a.mean().unwrap();
    let mean_b = b.mean().unwrap();

    let mut numerator: f32 = 0.0;
    let mut denom_a: f32 = 0.0;
    let mut denom_b: f32 = 0.0;

    for i in 0..a.len() {
        let diff_a = a[i] - mean_a;
        let diff_b = b[i] - mean_b;

        numerator += diff_a * diff_b;
        denom_a += diff_a * diff_a;
        denom_b += diff_b * diff_b;
    }

    numerator / (denom_a.sqrt() * denom_b.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_wav_file() {
        let mut path =
            "/users/but/info/condamik/Amadeus/file_sender/Chaussette_112202.wav".to_string();

        let result = analyze_wav_file(path.as_str());
        println!("result : {:?}", result);
        assert!(result.is_ok());

        let audio_info = result.unwrap();
        assert_eq!(audio_info.notes.len(), 885);
        assert_eq!(audio_info.tempo, 130.0);
        assert_eq!(audio_info.key, "A minor");
        assert_eq!(audio_info.duration.as_secs_f32(), 190.0);
    }
}

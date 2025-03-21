use std::{path::Path, process::exit};
use wavers::{Samples, Wav};

/// Converts a wav file to a buffer of i16 samples using wavers crate
///
/// # Arguments
///
/// * `path` - A path to the wav file
///
/// # Returns
///
/// A vector of i16 samples
pub fn wav_to_buffer(path: &Path) -> Vec<i16> {
    let mut wav: Wav<i16> = match Wav::from_path(path) {
        Ok(w) => w,
        Err(_) => {
            print!("Cannot read wav file");
            exit(1);
        }
    };

    let samples: Samples<i16> = match wav.read() {
        Ok(s) => s,
        Err(_) => {
            print!("Cannot read wav samples");
            exit(1);
        }
    };

    return samples.to_vec();
}

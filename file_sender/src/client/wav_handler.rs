use crate::errors::wav_error::WavError;
use std::path::Path;
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
pub fn wav_to_buffer(path: &Path) -> Result<Vec<i16>, WavError> {
    let mut wav: Wav<i16> = match Wav::from_path(path) {
        Ok(w) => w,
        Err(_) => {
            return Err(WavError {
                details: "Cannot open wav file".to_string(),
            })?;
        }
    };

    let samples: Samples<i16> = match wav.read() {
        Ok(s) => s,
        Err(_) => {
            return Err(WavError {
                details: "Cannot create samples".to_string(),
            })?;
        }
    };

    Ok(samples.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_wav_to_buffer() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("Chaussette_112202.wav");

        let buffer = match wav_to_buffer(&path) {
            Ok(b) => b,
            Err(e) => {
                println!("Error : {}", e);
                assert!(false);
                return;
            }
        };
        assert_eq!(buffer.len(), 16803072);
    }
}

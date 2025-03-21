use super::wav_handler::wav_to_buffer;
use crate::errors::connection_error::ConnectionError;
use std::path::Path;
use std::io::{BufWriter, Write};
use std::net::TcpStream;

/// Converts a vector of i16 samples to a vector of u8 samples
fn i16_to_u8_vec(samples: &[i16]) -> Vec<u8> {
    samples.iter().flat_map(|&s| s.to_le_bytes()).collect()
}

///
/// Sends a wav file to a server
///
/// # Arguments
/// * `data` - A path to the wav file
/// * `server` - A server address
/// * `bind` - A bind address
///
/// # Returns
/// A Result containing the number of bytes sent or an error
pub(crate) fn send_mp3(data: &Path, server: &str) -> Result<usize, ConnectionError> {
    let stream = match TcpStream::connect(server) {
        Ok(s) => s,
        Err(_) => {
            return Err(ConnectionError {
                details: "Cannot connect to server".to_string(),
            });
        }
    };
    let mut writer = BufWriter::new(stream);
    let data = wav_to_buffer(data);
    let data = i16_to_u8_vec(&data);
    writer.write_all(&data).map_err(|e| ConnectionError { details: e.to_string() })?;
    writer.flush().map_err(|e| ConnectionError { details: e.to_string() })?;
    Ok(data.len())
}

#[cfg(test)]
mod network_tests {
    use super::*;

    #[test]
    fn test_i16_to_u8_vec() {
        let samples = vec![1, 2, 3, 4];
        let result = i16_to_u8_vec(&samples);
        assert_eq!(result, vec![1, 0, 2, 0, 3, 0, 4, 0]);
    }
}

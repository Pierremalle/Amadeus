use super::socket::create_socket;
use super::wav_handler::wav_to_buffer;
use crate::errors::connection_error::ConnectionError;
use std::path::Path;

fn i16_to_u8_vec(samples: &[i16]) -> Vec<u8> {
    samples.iter().flat_map(|&s| s.to_le_bytes()).collect()
}

pub(crate) fn send_mp3(data: &Path, server: &str) -> Result<usize, ConnectionError> {
    let socket = create_socket();
    let data = wav_to_buffer(data);
    let data = i16_to_u8_vec(&data);
    let sombrero = match socket.send_to(&data, server) {
        Ok(s) => s,
        Err(_) => {
            return Err(ConnectionError {
                details: "Cannot send data".to_string(),
            });
        }
    };

    return Ok(sombrero);
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

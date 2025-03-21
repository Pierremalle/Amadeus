use std::net::UdpSocket;
use crate::errors::socket_error::SocketError;

/// Creates a UDP socket
///
/// # Arguments
///
/// * `addr` - A string slice that holds the address and port to bind
pub(crate) fn create_socket(addr:&str) -> Result<UdpSocket, SocketError> {
    let _: UdpSocket = match UdpSocket::bind(addr) {
        Ok(s) => {
            return Ok(s);
        }
        Err(_) => {
            print!("Cannot create socket");
            return Err(SocketError {
                details: "Cannot create socket".to_string(),
            })
        }
    };
}

#[cfg(test)]
mod socket_tests {
    use std::{net::{Ipv4Addr, SocketAddr, SocketAddrV4}, process::exit};

    use super::*;

    #[test]
    fn test_create_socket() {
        println!("test_create_socket");
        let s = match create_socket("0.0.0.0:5436") {
            Ok(s) => s,
            Err(_) => {
                assert!(false);
                exit(1);
            }
        };
        assert_eq!(s.local_addr().unwrap(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0), 5436)));
    }
        
}

use std::{net::UdpSocket, process::exit};

pub(crate) fn create_socket() -> UdpSocket {
    let _: UdpSocket = match UdpSocket::bind("0.0.0.0") {
        Ok(s) => {
            return s;
        }
        Err(_) => {
            print!("Cannot connect to socket");
            exit(1);
        }
    };
}

#[cfg(test)]
mod socket_tests {
    use super::*;
}

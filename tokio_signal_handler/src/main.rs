use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buffer = [0; 1024];

    loop {
        let (len, addr) = sock.recv_from(&mut buffer).await?;
        let buffer = &buffer[..len];
        println!("Received {} bytes from {}", buffer.len(), addr);
        println!("Data: {:?}", buffer);
    }
}

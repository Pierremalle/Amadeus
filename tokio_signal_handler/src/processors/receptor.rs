use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn get_datas(socket: &mut TcpStream, vector: &mut Vec<i16>) {
    let mut buffer: Vec<u8> = vec![0u8; 4096];
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => {
                for chunk in buffer[..n].chunks_exact(2) {
                    let value = i16::from_ne_bytes([chunk[0], chunk[1]]);
                    vector.push(value);
                }
                println!("Received {} bytes, total {} values", n, vector.len());
            }
            Err(e) => {
                eprintln!("Read error: {:?}", e);
                break;
            }
        }
    }
}

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn process(mut socket: TcpStream) {
    let mut buffer = vec![0u8; 4096];
    let mut datas: Vec<i16> = Vec::new();

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => {
                for chunk in buffer[..n].chunks_exact(2) {
                    let value = i16::from_ne_bytes([chunk[0], chunk[1]]);
                    datas.push(value);
                }
                println!("Received {} bytes, total {} values", n, datas.len());
            }
            Err(e) => {
                eprintln!("Read error: {:?}", e);
                break;
            }
        }
    }

    println!("Received all data: {:?}", datas.len());
    socket.shutdown().await.unwrap();
}

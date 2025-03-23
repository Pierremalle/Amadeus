use serde_json::json;
use std::env;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn process(mut socket: TcpStream) {
    let mut datas: Vec<i16> = Vec::new();

    get_datas(&mut socket, &mut datas).await;

    println!("Received all data: {:?}", datas.len());

    let server_addr: String = env::var("BACKEND_ADDR").unwrap_or_else(|_| "localhost".to_string());
    let server_port: String = env::var("API_PORT").unwrap_or_else(|_| "8000".to_string());
    let url = format!("{}:{}", server_addr, server_port);

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json"),
    );
    let body = json!({
        "data": datas,
    });

    let client = reqwest::Client::new();
    let resp = match client.post(url).headers(headers).json(&body).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            socket.shutdown().await.unwrap();
            return;
        }
    };
    println!("{:#?}", resp);

    socket.shutdown().await.unwrap();
}

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

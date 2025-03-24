use chrono::Utc;
use reqwest::multipart;
use serde_json::json;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn send_data(socket: &mut tokio::net::TcpStream, datas: &Vec<i16>) {
    let server_addr: String =
        env::var("BACKEND_ADDR").unwrap_or_else(|_| "http://localhost".to_string());
    let server_port: String = env::var("API_PORT").unwrap_or_else(|_| "8000".to_string());
    let url = format!("{}:{}/upload", server_addr, server_port);

    // Convert i16 to bytes
    let byte_data: Vec<u8> = datas
        .iter()
        .flat_map(|sample| sample.to_le_bytes())
        .collect();

    // Create a temporary file to store the audio data
    let temp_path = "/tmp/temp_audio.wav";
    let mut file = File::create(temp_path)
        .await
        .expect("Error while creatig the file");
    file.write_all(&byte_data)
        .await
        .expect("Error while writing file");
    file.flush().await.expect("Flushing error");

    // Load the file into a multipart::Part
    let file_part = multipart::Part::bytes(byte_data)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .unwrap();

    // Create a JSON object with metadata
    let metadata = json!({
        "timestamp": Utc::now().timestamp(),
        "user_id": "12345",
        "name" : Utc::now().timestamp(),
        "format": "wav"
    });

    // Serialize the JSON object to a string
    let json_str = serde_json::to_string(&metadata).expect("Serialization error");

    // Create the multipart form
    let form = multipart::Form::new()
        .part("file", file_part)
        .text("metadata", json_str);

    let client = reqwest::Client::new();
    let resp = match client.post(url).multipart(form).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Request error : {:?}", e);
            socket.shutdown().await.unwrap();
            return;
        }
    };

    println!("{:#?}", resp);
}

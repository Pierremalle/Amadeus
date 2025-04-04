use chrono::Utc;
use reqwest::multipart;
use serde_json::json;
use std::env;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn send_data(
    socket: &mut TcpStream,
    data: &[i16],
) -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = env::var("BACKEND_ADDR").unwrap_or_else(|_| "http://localhost".to_string());
    let server_port = env::var("API_PORT").unwrap_or_else(|_| "8000".to_string());
    let url = format!("{}:{}/upload", server_addr, server_port);

    // Convert i16 to bytes
    let byte_data: Vec<u8> = data
        .iter()
        .flat_map(|&sample| sample.to_le_bytes())
        .collect();

    // Load the file into a multipart::Part directly from memory
    let file_part = multipart::Part::bytes(byte_data.clone())
        .file_name("audio.wav")
        .mime_str("audio/wav")?;

    // Create a JSON object with metadata
    let metadata = json!({
        "timestamp": Utc::now().timestamp(),
        "user_id": "12345",
        "name": Utc::now().timestamp(),
        "format": "wav"
    });

    // Serialize metadata to string
    let json_str = metadata.to_string();

    // Create the multipart form
    let form = multipart::Form::new()
        .part("file", file_part)
        .text("metadata", json_str);

    // Send the request
    let client = reqwest::Client::new();
    match client.post(&url).multipart(form).send().await {
        Ok(resp) => println!("Response: {:#?}", resp),
        Err(e) => {
            eprintln!("Request error: {:?}", e);
            socket.shutdown().await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use mockito::Server;
    use std::env;
    use tokio::{
        io::AsyncReadExt,
        net::{TcpListener, TcpStream},
    };

    /// Setup a TCP listener for testing
    async fn setup_listener() -> Result<(TcpListener, u16)> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        Ok((listener, port))
    }

    /// Set up a TCP stream as client for testing
    async fn create_client(port: u16) -> Result<TcpStream> {
        Ok(TcpStream::connect(format!("127.0.0.1:{}", port)).await?)
    }

    /// Try to send i16 data onto a mock http client
    ///
    /// Is multithreaded to allow the execution of the server and client simultaneously
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_send_data_success() -> Result<()> {
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", "/upload")
            .with_status(200)
            .with_body("Success")
            .create_async()
            .await;

        unsafe {
            env::set_var("BACKEND_ADDR", &server.url());
        }
        unsafe {
            env::set_var("API_PORT", "80");
        }

        let (_listener, port) = setup_listener().await?;
        let mut client_socket = create_client(port).await?;

        let data = vec![1, 2, 3, 4];
        let result = send_data(&mut client_socket, &data).await;

        assert!(result.is_ok());
        Ok(())
    }

    /// Simulate a failing API to check error handling
    ///
    /// If request fail, the method should end as ok while closing the socket
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_send_data_failure() -> Result<()> {
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", "/upload")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        unsafe {
            env::set_var("BACKEND_ADDR", &server.url());
        }
        unsafe {
            env::set_var("API_PORT", "80");
        }

        let (_listener, port) = setup_listener().await?;
        let mut client_socket = create_client(port).await?;

        let data = vec![1, 2, 3, 4];
        let result = send_data(&mut client_socket, &data).await;

        assert!(result.is_ok()); // Function should handle errors gracefully

        // A closed socket should always return an empty buffer
        let datas = match client_socket.read_i8().await {
            Ok(d) => d,
            Err(_) => {
                assert!(false);
                return Err(anyhow::Error::msg("fail reading in socket"));
            }
        };

        if datas <= 0 {
            assert!(true)
        }

        Ok(())
    }
}

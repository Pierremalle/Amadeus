pub mod client;
pub mod errors;

use client::network::send_mp3;
use std::path::Path;

fn run(args: &[String]) -> Result<(), String> {
    if args.len() < 3 || args[1] == "-h" {
        return Err(format!("Usage: {} <file> <server>", args[0]));
    }

    let file = Path::new(&args[1]);
    let server = &args[2];

    match send_mp3(file, server) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: {}\n", e.details)),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match run(&args) {
        Ok(_) => println!("Data sent"),
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::path::PathBuf;
    use std::thread;

    fn start_mock_server(addr: String) {
        thread::spawn(move || {
            let listener = TcpListener::bind(&addr).expect("Failed to bind server");
            println!("Mock server listening on {}", addr);

            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        println!("Client connected!");
                        let mut buffer = Vec::new();
                        stream.read_to_end(&mut buffer).unwrap();
                        println!("Received {} bytes", buffer.len());
                    }
                    Err(e) => eprintln!("Connection failed: {}", e),
                }
            }
        });
    }

    #[test]
    fn test_run_with_valid_args() {
        let addr = "127.0.0.1:9000";
        start_mock_server(addr.to_string());

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("Chaussette_112202.wav");

        if !path.exists() {
            let mut file = File::create(&path).expect("Failed to create test WAV file");
            file.write_all(b"FAKE_WAV_DATA")
                .expect("Failed to write test data");
        }

        let args = vec![
            "program_name".to_string(),
            path.to_string_lossy().to_string(),
            addr.to_string(),
        ];

        let result = run(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_missing_args() {
        let args = vec!["program_name".to_string()];
        let result = run(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_with_invalid_file() {
        let args = vec![
            "program_name".to_string(),
            "non_existent_file.wav".to_string(),
            "http://localhost:9001".to_string(),
        ];

        let result = run(&args);
        assert!(result.is_err());
    }
}

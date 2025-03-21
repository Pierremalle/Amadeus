pub mod processors;

use std::{env, io::Error, process::exit};

use tokio::net::TcpListener;
use processors::process;

#[tokio::main]
async fn main() {
    console_subscriber::init();

    let port: String = env::var("TOKIO_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind = format!("0.0.0.0:{}", port);


    let listener = match TcpListener::bind(bind).await {
        Ok(l) => l,
        Err(_) => {
            println!("Cannot create the server.");
            exit(1);
        }
    };

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

pub mod processors;

use std::{env, process::exit};

use processors::process::process;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    console_subscriber::init();

    let port: String = env::var("TOKIO_PORT").unwrap_or_else(|_| "9000".to_string());
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

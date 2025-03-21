pub mod processors;

use tokio::net::TcpListener;
use processors::process;

#[tokio::main]
async fn main() {
    console_subscriber::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == "help" {
            println!("Usage: {} [<server>] [<bind>]", args[0]);
            std::process::exit(1);
        }
    }

    let server = if args.len() == 2 { args[1].to_string() } else { "51.75.126.107:8000".to_string()};
    let bind = if args.len() == 3 { &args[2] } else { "0.0.0.0:9000"};

    let listener = TcpListener::bind(bind).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let server_clone = server.clone();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket, server_clone).await;
        });
    }
}
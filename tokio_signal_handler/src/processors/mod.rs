use tokio::net::TcpStream;

pub async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    socket.readable().await.unwrap();
    println!("Accepted connection");
}
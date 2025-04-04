#[test]
async fn test_socket_binding() {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buffer = [0; 1024];
}

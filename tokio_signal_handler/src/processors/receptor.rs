use tokio::{io::AsyncReadExt, net::TcpStream};

/// Get all datas from a socket has a Vec of i16
///
/// Now handles only wav datas gathered by wavers crate
///
/// # Arguments
/// # `socket` - The listening socket opened by tokio
/// # `vector` - The mutable reference of the vector in which store the datas
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio::{
        io::AsyncWriteExt,
        net::{TcpListener, TcpStream},
    };

    /// Setup a listener to listen during the tests
    async fn setup_listener() -> Result<(TcpListener, u16)> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        Ok((listener, port))
    }

    /// Create a client stream to send datas during tests
    async fn create_client(port: u16) -> Result<TcpStream> {
        Ok(TcpStream::connect(format!("127.0.0.1:{}", port)).await?)
    }

    /// Try to get datas from a socket
    ///
    /// Sending u8 datas should concatenate them to create i16 datas
    #[tokio::test]
    async fn test_get_datas_with_valid_data() -> Result<()> {
        let (listener, port) = setup_listener().await?;
        let mut vector = Vec::new();

        tokio::spawn(async move {
            if let Ok((mut server_socket, _)) = listener.accept().await {
                let data: Vec<u8> = vec![1, 0, 2, 0, 3, 0, 4, 0];
                let _ = server_socket.write_all(&data).await;
            }
        });

        let mut client_socket = create_client(port).await?;
        get_datas(&mut client_socket, &mut vector).await;

        assert_eq!(vector, vec![1, 2, 3, 4]);
        Ok(())
    }

    /// Create a socket and check if sending no datas work
    #[tokio::test]
    async fn test_get_datas_with_empty_data() -> Result<()> {
        let (listener, port) = setup_listener().await?;
        let mut vector = Vec::new();

        tokio::spawn(async move {
            if let Ok((_server_socket, _)) = listener.accept().await {
                // Send no data
            }
        });

        let mut client_socket = create_client(port).await?;
        get_datas(&mut client_socket, &mut vector).await;

        assert!(vector.is_empty());
        Ok(())
    }

    /// Test to send odd number of bytes
    ///
    /// Last part of data should be ignored since it can't be concatenated with another
    #[tokio::test]
    async fn test_get_datas_with_odd_number_of_bytes() -> Result<()> {
        let (listener, port) = setup_listener().await?;
        let mut vector = Vec::new();

        tokio::spawn(async move {
            if let Ok((mut server_socket, _)) = listener.accept().await {
                let data: Vec<u8> = vec![1, 0, 2, 0, 3];
                let _ = server_socket.write_all(&data).await;
            }
        });

        let mut client_socket = create_client(port).await?;
        get_datas(&mut client_socket, &mut vector).await;

        assert_eq!(vector, vec![1, 2]);
        Ok(())
    }
}

use serde_json::json;
use std::env;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::processors::{receptor::get_datas, sender::send_data};

/// Process method handling app logic
///
/// Gather data from the socket and send data to the server
pub async fn process(mut socket: TcpStream) {
    let mut datas: Vec<i16> = Vec::new();

    get_datas(&mut socket, &mut datas).await;

    println!("Received all data: {:?}", datas.len());

    let _ = match send_data(&mut socket, &*datas).await {
        Ok(_) => println!("Data sending successfull"),
        Err(_) => println!("Cannot send data to server"),
    };

    socket.shutdown().await.unwrap();
}

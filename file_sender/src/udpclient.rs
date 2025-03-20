use std::{fs::File, net::UdpSocket, process::exit};

fn create_socket() -> UdpSocket {
    let socket: UdpSocket = match UdpSocket::bind("0.0.0.0") {
        Ok(s) => {
            return s;
        }
        Err(_) => {
            print!("Cannot connect to socket");
            exit(1);
        }
    };

    socket
}

fn send_mp3(data:File,server:&str) ->Result<usize> {
    let socket = create_socket();
    let sombrero = match Result<usize> socket.send_to(data,server){
        Ok(s) =>{
            return s;
        }
        Err(_) =>{
            exit(2)
        }
    };
        
}


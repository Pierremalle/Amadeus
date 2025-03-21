pub mod client;
pub mod errors;

use client::network::send_mp3;
use std::path::Path;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <file> <server> [<bind>]", args[0]);
        std::process::exit(1);
    }

    let file = Path::new(&args[1]);
    let server = &args[2];
    let bind = if args.len() == 4 { &args[3] } else { "0.0.0.0:5436"};
    
    match send_mp3(file, server, bind) {
        Ok(_) => println!("Data sent"),
        Err(e) => println!("Error: {}\n", e.details),
    }
}

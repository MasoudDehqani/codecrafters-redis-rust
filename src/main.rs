// use std::{
//     io::{Read, Write},
// net::TcpListener,
//     thread,
// };

use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{io::Read, net::TcpListener};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    // net::TcpListener,
};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Mutex::new(stream);
                let stream = Arc::new(stream);
                let stream = Arc::clone(&stream);

                thread::spawn(move || loop {
                    let mut buffer = [0_u8; 512];
                    stream.lock().unwrap().read(&mut buffer).unwrap();
                    stream.lock().unwrap().write_all(b"+PONG\r\n").unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

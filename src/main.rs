// use std::{
//     io::{Read, Write},
// net::TcpListener,
//     thread,
// };

use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{io::Read, net::TcpListener};

fn main() {
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

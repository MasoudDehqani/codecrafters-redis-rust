// use std::{
//     io::{Read, Write},
// net::TcpListener,
//     thread,
// };

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((mut stream, _socket_addr)) => loop {
                let mut buffer = [0_u8; 512];
                stream.read(&mut buffer).await.unwrap();
                stream.write_all(b"+PONG\r\n").await.unwrap();
            },

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => loop {
    //             thread::spawn(|| {
    //                 let mut buffer = [0_u8; 512];
    //                 stream.read(&mut buffer).unwrap();
    //                 stream.write_all(b"+PONG\r\n").unwrap();
    //             });
    //         },
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}

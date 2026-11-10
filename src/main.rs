#[allow(unused_imports)]
use std::net::TcpListener;
use std::{io::{Read, Write}, net::TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    //
     let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
     for stream in listener.incoming() {
         match stream {
             Ok(stream) => {
                 println!("accepted new connection");
                 handle_write(stream);

             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}


fn handle_write(mut stream: TcpStream) {
    // let out = "HTTP/1.1 200 OK\r\n\r\n";
    // Storing the incoming stream in a buffer
    let mut buf = [0u8;1024];

    // read() puts the received bytes into the buffer 
    // and returns the number of bytes

    let bytes_read = stream.read(&mut buf).unwrap();

    // Reads the filled buffer and converts bytes to string 
    let request = std::str::from_utf8(&buf[..bytes_read]).unwrap();

    let mut part = request.split_whitespace();

    let _method = part.next().unwrap();
    let path = part.next().unwrap();

    let response = if path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 Not found\r\n\r\n"
    };


    stream.write_all(response.as_bytes()).unwrap();
}

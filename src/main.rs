#[allow(unused_imports)]
use std::net::TcpListener;
use std::{ io::{Read, Write}, net::TcpStream};
use std::thread;
use std::sync::Arc;
use std::path::{Path,PathBuf};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Collect command line arguments
    let args: Vec<String> =  std::env::args().collect();

    // Extract the directory from the arguments
    let directory = match args.get(2) {
        Some(dir) => dir.clone(),
        None => { println!("Missing argument");
                return;
        }
    };
    // Give each thread shared ownership of the directory
    let directory = Arc::new(PathBuf::from(directory));

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
     for stream in listener.incoming() {
         match stream {
             Ok(stream) => {
                 println!("accepted new connection");
                 let directory = Arc::clone(&directory);
                 // as_path() converts the PathBuf reference to Path reference
                 thread::spawn(move || {
                    handle_write(stream, directory.as_path());
                });

             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}


fn handle_write(mut stream: TcpStream,dir: &Path) {
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

    let mut user_agent = "";

    // Extract the echo path using strip_prefix(). 
    let echo_string = path.strip_prefix("/echo/");

    // Extract the file from the path
    let file_path = path.strip_prefix("/files/");


    for line in request.lines() {
        match line.split_once(":") {
            Some((key,value)) => {
                if key.trim().eq_ignore_ascii_case("user-agent") {
                    user_agent = value.trim();
                }
            }
            None => {}
        }
    } 
    if let Some(filename) = file_path {
        let full_path = dir.join(filename);
        
        let contents = std::fs::read(&full_path);

        match contents {
            Ok(data) => {
                println!("File successfully read");
                println!("File length {}",data.len());
            }
            Err(e) => {
                println!{"Could not read the file {}", e};
            }
        }
    }
    let response = if path == "/" {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    } else if let Some(body) = echo_string {
                format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", 
                        body.len(),
                        body
                )
    } else if path == "/user-agent" {
                format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", 
                        user_agent.len(),
                        user_agent
                )
        } else {
        String::from("HTTP/1.1 404 Not found\r\n\r\n")
    };

    stream.write_all(response.as_bytes()).unwrap();
}

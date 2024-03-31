use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_incoming_connection(mut stream: TcpStream) {
    let read_stream = stream.try_clone().expect("Stream should be clonable");
    let reader = BufReader::new(read_stream);

    for line in reader.lines() {
        match line {
            Ok(command) => {
                println!("Received {}", command);
                if command == "ping" {
                    stream
                        .write_all("+PONG\r\n".as_bytes())
                        .expect("data written");
                    println!("data sent");
                }
            }
            Err(e) => {
                println!("Failure {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_incoming_connection(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

#[allow(unused_imports)]
use std::{
    io::{BufRead, BufReader, Error, ErrorKind, Write},
    net::{TcpListener, TcpStream},
    process::Command,
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

/// Run server TCP for run test
#[test]
fn test_handle_incoming_connection() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo -e 'ping' | redis-cli")
        .output()
        .expect("failed to execute process");

    let pong = String::from_utf8(output.stdout).unwrap();
    assert_eq!(pong, "PONG\n");
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

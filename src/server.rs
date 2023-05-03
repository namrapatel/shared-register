use std::sync::{Arc};
use std::thread;

use std::net::TcpListener;
use std::io::{Read, Write};
use crate::shared_register::AtomicRegister;

fn handle_client(mut stream: std::net::TcpStream, atomic_register: Arc<AtomicRegister>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let mut iter = request.trim().split_whitespace();
    let command = iter.next().unwrap();
    // TODO: test reading string from message
    let response = match command {
        "/read" => atomic_register.read(),
        "/write" => {
            let value = iter.next().unwrap_or("new value");
            atomic_register.write(String::from(value))
        },
        "/write_with_quorum" => {
            let value = iter.next().unwrap_or("new value");
            atomic_register.write_with_quorum(String::from(value))
        },
        _ => String::from("Invalid request"),
    };
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// TODO: Need ot use Arc here because we need different threads of operations to see the same AtomicRegister
pub fn start_server(port: u32, atomic_register: Arc<AtomicRegister>) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let atomic_register = atomic_register.clone();
                thread::spawn(move || {
                    handle_client(stream, atomic_register);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
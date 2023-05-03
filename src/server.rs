use std::sync::{Arc, Mutex};
use std::thread;

use std::net::TcpListener;
use std::io::{Read, Write};
use crate::shared_register::AtomicRegister; // Import the AtomicRegister from shared_register.rs

fn handle_client(mut stream: std::net::TcpStream, atomic_register: Arc<AtomicRegister>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let response = match request.trim() {
        "/read" => atomic_register.read(),
        "/write" => atomic_register.write(String::from("new value")),
        "/write_with_quorum" => atomic_register.write_with_quorum(String::from("new value")),
        _ => String::from("Invalid request"),
    };
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn start_server(port: u32, atomic_register: Arc<AtomicRegister>) {
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
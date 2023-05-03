use std::sync::{Arc, atomic::{AtomicPtr}};
use std::thread;

mod shared_register;
mod server;

use crate::shared_register::AtomicRegister;
use crate::server::start_server;

// Trying to demo usage of multiple servers running a shared AtomicRegister
fn main() {
    let nodes: Vec<String> = vec!["localhost:8000", "localhost:8001", "localhost:8002", "localhost:8003"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let register = Arc::new(AtomicPtr::new(Box::into_raw(Box::new("".to_string()))));

    // TODO: only two servers are started even though the loop is ran 4 times
    for i in 0..nodes.len() {
        println!("Entered");
        let nodes = nodes.clone();
        let register = register.clone();
        let atomic_register = Arc::new(AtomicRegister::new(i, nodes.clone(), register.clone()));
        let port = 8000 + i as u32; // TODO: might be a cleaner way to deal with i here
        println!("Port: {}", port);
        thread::spawn(move || {
            println!("Starting server2");
            let _server = start_server(port, atomic_register);
            println!("Server listening on port {}", port);
        });
    }

}
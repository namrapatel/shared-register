use std::sync::{Arc, Mutex};
use std::thread;

mod shared_register;
mod server;

use crate::shared_register::AtomicRegister;
use crate::server::start_server;

fn main() {
    let nodes: Vec<String> = vec!["localhost:8000", "localhost:8001", "localhost:8002", "localhost:8003"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}
use std::collections::HashMap;
use std::sync::{Arc};
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicPtr, Ordering};

type NodeId = usize;

#[derive(Clone)]
pub struct AtomicRegister {
    id: NodeId,
    nodes: Vec<String>,
    register: Arc<AtomicPtr<String>>,
}

impl AtomicRegister {
    pub fn new(id: NodeId, nodes: Vec<String>, register: Arc<AtomicPtr<String>>) -> Self {
        Self {
            id,
            nodes,
            register,
        }
    }

    pub fn read(&self) -> String {
        let mut register = self.register.load(Ordering::SeqCst);

        // TODO: this should make a read operation not return in so far as there are write operations changing the value
        loop {
            let latest_register = self.register.load(Ordering::SeqCst);
            if register == latest_register {
                break;
            }
            register = latest_register;
        }
        unsafe { (*register).clone() }
    }

    pub fn write(&self, value: String) -> String {
        println!("Value received in write: {}", value);
        let mut register = self.register.load(Ordering::SeqCst);
        let new_register = AtomicPtr::new(Box::into_raw(Box::new(value)));
        loop {
            match self.register.compare_exchange(register, new_register.load(Ordering::SeqCst), Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(current_register) => {
                    register = current_register;
                }
            }
        }

        "ACK".to_string()
    }

    pub fn write_with_quorum(&self, value: String) -> String {
        println!("Value received in write_with_quorum: {}", value);
        let mut ack_count = 0;
        let mut responses = HashMap::new();
        let quorum_size = (self.nodes.len() / 2) + 1;
    
        for node in &self.nodes {
            let url = format!("http://{}/write", node);
            let client = reqwest::blocking::Client::new();
            let response = client
                .post(&url)
                .json(&value)
                .send()
                .unwrap()
                .text()
                .unwrap();
            responses.insert(node, response);
        }
    
        let timeout_duration = Duration::from_secs(5); // wait for 5 seconds
        let start_time = Instant::now();
    
        loop {
            let mut updated_responses = HashMap::new();
    
            for (node, response) in &responses {
                if response == "ACK" {
                    ack_count += 1;
                }
                if ack_count >= quorum_size {
                    return self.write(value);
                }
                let url = format!("http://{}/read", node);
                let client = reqwest::blocking::Client::new();
                let response = client
                    .get(&url)
                    .send()
                    .unwrap()
                    .text()
                    .unwrap();
                updated_responses.insert(node.clone(), response);
            }
    
            responses = updated_responses;
    
            if start_time.elapsed() >= timeout_duration {
                break;
            }
    
            std::thread::sleep(Duration::from_millis(100));
        }
    
        // If we reach here, we did not receive a quorum of responses within the timeout duration
        "ERROR: Quorum not reached".to_string()
    }
}


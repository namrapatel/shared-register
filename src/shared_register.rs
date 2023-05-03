use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
        unsafe { (*register).clone() }
    }

    pub fn write_with_quorum(&self, value: String) -> String {
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

        loop {
            let mut updated_responses = HashMap::new();

            for (node, response) in &responses {
                if response == "ack" {
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
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}


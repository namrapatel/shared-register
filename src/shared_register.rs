use std::collections::HashMap;
use std::time::Duration;

type NodeId = usize;

pub struct AtomicRegister {
    value: String,
    quorum_size: usize,
    nodes: HashMap<NodeId, String>,
}

impl AtomicRegister {
    fn new(quorum_size: usize) -> Self {
        AtomicRegister {
            value: String::new(),
            quorum_size,
            nodes: HashMap::new(),
        }
    }

    fn read(&self) -> String {
        self.value.clone()
    }

    fn write(&mut self, value: String) -> String {
        self.value = value.clone();
        value
    }

    async fn write_with_quorum(&mut self, value: String) -> String {
        let mut ack_count = 0;
        let mut responses = HashMap::new();

        for (node_id, node_url) in &self.nodes {
            let client = reqwest::Client::new();
            let url = format!("http://{}/write", node_url);
            let value_clone = value.clone();

            let response = client.post(&url)
                .body(value_clone.clone())
                .send();

            responses.insert(node_id.clone(), response);
        }

        loop {
            for future in &mut responses {
                // Await the future to get the result
                match future.1.await {
                    Ok(res) => {
                        if res.status().is_success() {
                            ack_count += 1;
                        }
                    }
                    Err(_) => {}
                }
            }

            if ack_count >= self.quorum_size {
                break;
            }

            std::thread::sleep(Duration::from_millis(100));
        }

        self.write(value)
    }
}
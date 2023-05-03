use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type NodeId = usize;

struct AtomicRegister {
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
}
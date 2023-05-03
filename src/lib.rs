pub mod shared_register;
pub mod server;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, atomic::{AtomicPtr}};
    use std::thread;
    use std::time::Duration;
    use reqwest::blocking::Client;


    use crate::shared_register::AtomicRegister;
    use crate::server::start_server;
    
    #[test]
    fn test_handle_client() {
        let register = Arc::new(AtomicPtr::new(Box::into_raw(Box::new("".to_string()))));
        let atomic_register = Arc::new(AtomicRegister::new(0, vec!["127.0.0.1:8080".to_string()], register.clone()));
        thread::spawn(move || {
            start_server(8080, atomic_register.clone());
        });
        thread::sleep(Duration::from_millis(100));
        let client = Client::new();
        let response = client.post("http://127.0.0.1:8080/write").body("test message").send().unwrap_or_else(|err| {
            eprintln!("Error sending request: {}", err);
            panic!("Failed to send request");
        }).text().unwrap();
        println!("Response from server: {}", response);
        assert_eq!(response, "ACK");
    }

    fn test_handle_client_read() {
        let register = Arc::new(AtomicPtr::new(Box::into_raw(Box::new("".to_string()))));
        let atomic_register = Arc::new(AtomicRegister::new(0, vec!["127.0.0.1:8080".to_string()], register.clone()));
        thread::spawn(move || {
            start_server(8080, atomic_register.clone());
        });
        thread::sleep(Duration::from_millis(100));
        let client = Client::new();
        let write_response = client.post("http://127.0.0.1:8080/write").body("test message").send().unwrap_or_else(|err| {
            eprintln!("Error sending request: {}", err);
            panic!("Failed to send request");
        }).text().unwrap();
        println!("Response from server: {}", write_response);
        assert_eq!(write_response, "ACK");
        let read_response = client.get("http://127.0.0.1:8080/read").send().unwrap_or_else(|err| {
            eprintln!("Error sending request: {}", err);
            panic!("Failed to send request");
        }).text().unwrap();
        println!("Response from server: {}", read_response);
        assert_eq!(read_response, "test message");
    }
}
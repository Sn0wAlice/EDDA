use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use serde_json::json;

fn main() {
    let server_address = "127.0.0.1:27998";

    loop {
        // Attempt to connect to the server
        match TcpStream::connect(server_address) {
            Ok(mut stream) => {
                println!("Successfully connected to server at {}", server_address);

                // Spawn a new thread to handle incoming messages from the server
                let mut read_stream = stream.try_clone().expect("Failed to clone stream for reading");
                let server_address_clone = server_address.to_string();
                thread::spawn(move || {
                    let mut buffer = [0; 1024];
                    loop {
                        match read_stream.read(&mut buffer) {
                            Ok(n) if n > 0 => {
                                let response = String::from_utf8_lossy(&buffer[..n]);
                                println!("Received message from server: {}", response);
                            }
                            Ok(_) => {
                                // Connection closed by the server
                                println!("Server closed the connection, attempting to reconnect...");
                                break;
                            }
                            Err(e) => {
                                eprintln!("Error reading from server: {}", e);
                                break;
                            }
                        }
                    }

                    // Connection lost; attempt to reconnect
                    println!("Attempting to reconnect to {}", server_address_clone);
                });

                // Prepare and send an initial JSON message
                let message = json!({
                    "event_type": "hello",
                    "payload": {
                        "message": "Hello, server!"
                    },
                    "identifier_source": "ece4985f-e3de-4453-b9a7-c517e524d555",
                    "priority": 1,
                    "ttl": 0
                });
                let message_string = message.to_string();
                if let Err(e) = stream.write_all(message_string.as_bytes()) {
                    eprintln!("Failed to send message: {}", e);
                    continue;
                }

                println!("Sent 'hello' message to the server");

                // Keep the main thread active for additional messaging (optional)
                loop {
                    // Simulate more actions or periodic checks if needed
                    // Example: Send another message periodically
                    thread::sleep(Duration::from_secs(10)); // Adjust frequency as needed
                    if let Err(_) = stream.write_all(message_string.as_bytes()) {
                        eprintln!("Connection lost, exiting main thread loop...");
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}. Retrying in 5 seconds...", e);
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}
use uuid::Uuid;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use serde_json::{json, Value};
use rand::seq::SliceRandom;

use crate::helper::mainstruct::CorePacket;
use crate::helper::trace::logs;


pub struct TcpClient {
    uuid: Uuid,
    stream: TcpStream,

    is_consumer: bool,
    is_producer: bool,

    consumer_type: String,
}

pub struct TcpServer {
    clients: Arc<Mutex<HashMap<Uuid, TcpClient>>>,
    listener: TcpListener
}

impl TcpServer {

    // Instantiate a new TcpServer
    fn new(addr: &str) -> std::io::Result<TcpServer> {
        let listener = TcpListener::bind(addr)?;
        println!("TCP Server listening on {}", addr);
        let clients = Arc::new(Mutex::new(HashMap::new()));
        let server = Self {
            clients: clients.clone(),
            listener: listener.try_clone()?, // Clone the listener to avoid moving it
        };
        Ok(server)
    }

    fn handle_client(&self, mut stream: TcpStream, client_uuid: Uuid) {
        let mut buf = [0; 1024];
        loop {
            match stream.read(&mut buf) {
                Ok(n) if n > 0 => {
                    if n == 1 {
                        // Ignore empty messages
                        continue;
                    }
                    logs(json!({"message": format!("Received {} bytes from client {}", n, client_uuid)}), 1);
                    // show the uuid of the client and the decoded message
                    let message = String::from_utf8_lossy(&buf[..n]);
                    //println!("Client UUID: {}, Message: {}", client_uuid, message);

                    // try to convert to Value
                    let value: Result<Value, _> = serde_json::from_str(&message);
                    let json_value = match value {
                        Ok(v) => v,
                        Err(e) => {
                            println!("Error parsing JSON: {}", e);
                            continue;
                        }
                    };

                    // check if object have index "client_type"
                    if json_value.get("client_type").is_none() {
                        // convert to CorePacket
                        let mut core_packet = CorePacket::from_json(json_value);
                        //println!("CorePacket: {:?}", core_packet);

                        // find the client in the hashmap who is a consumer and where consumer_type = core_packet.event_type
                        let clients = self.clients.lock().unwrap();
                        let available_clients = clients.iter().filter(|(_, client)| client.is_consumer && client.consumer_type == core_packet.event_type).collect::<Vec<_>>();

                        // check if the client is available
                        if available_clients.len() > 0 {
                            
                            // choose client randomly with rand
                            let random_client = available_clients.choose(&mut rand::thread_rng()).unwrap();

                            core_packet.identifier_destination = random_client.1.uuid;
                            // send the message to the client
                            if let Err(e) = random_client.1.stream.try_clone().expect("Failed to clone stream").write_all(&core_packet.as_bytes()) {
                                println!("Error writing to client {}: {}", random_client.0, e);
                            }

                        } else {
                            println!("No available clients for event type: {}", core_packet.event_type);
                        }


                        // free the lock
                        drop(clients);
                    } else if json_value.get("client_type").unwrap().as_str().unwrap() == "consumer" {
                        // set TcpClient as consumer and set the consumer type
                        let mut clients = self.clients.lock().unwrap();
                        let client = clients.get_mut(&client_uuid).unwrap();
                        client.is_consumer = true;
                        client.is_producer = false;
                        client.consumer_type = json_value.get("consumer_type").unwrap().as_str().unwrap().to_string();
                        // free the lock
                        drop(clients);
                    }

                }
                Ok(_) | Err(_) => {
                    // On EOF or error, remove the client from the hashmap
                    let mut clients = self.clients.lock().unwrap();
                    clients.remove(&client_uuid);
                    break;
                }
            }
        }
    }

    pub fn start(self) -> std::io::Result<()> {
        
        // start a new thread to listen for incoming connections
        thread::spawn(move || {
            for stream in self.listener.incoming() {
                let stream = stream.expect("Failed to accept connection");
                let client_uuid = uuid::Uuid::new_v4();
                logs(json!({"message": format!("New client connected with UUID: {}", client_uuid)}), 1);
        
                // Store the client in the hashmap
                let clients_clone = Arc::clone(&self.clients);
                let self_clone = self.clone(); // Clone the server for each thread
                let mut clients = clients_clone.lock().unwrap();

                let new_client:TcpClient = TcpClient {
                    uuid: client_uuid,
                    stream: stream.try_clone().expect("Failed to clone stream"),
                    is_consumer: false,
                    is_producer: true,
                    consumer_type: String::from(""),
                };

                clients.insert(client_uuid, new_client);
        
                // unlock the clients
                drop(clients);
        
                // Spawn a new thread to handle the client
                thread::spawn(move || {
                    self_clone.handle_client(stream, client_uuid);
                });
            }
        });

        Ok(())
    }
    
    pub fn get_clients(&self) -> Vec<Uuid> {
        self.clients.lock().unwrap().keys().cloned().collect()
    }

    pub fn send_hello_to_client(&self, client_uuid: Uuid) {
        let clients = self.clients.lock().unwrap();
        if let Some(client) = clients.get(&client_uuid) {
            let message = "Hello from server";
            if let Err(e) = client.stream.try_clone().expect("Failed to clone stream").write_all(message.as_bytes()) {
                println!("Error writing to client {}: {}", client_uuid, e);
            }
        }
    }

    pub fn send_to_client(&self, client_uuid: Uuid, message: &str) {
        let clients = self.clients.lock().unwrap();
        if let Some(client) = clients.get(&client_uuid) {
            if let Err(e) = client.stream.try_clone().expect("Failed to clone stream").write_all(message.as_bytes()) {
                println!("Error writing to client {}: {}", client_uuid, e);
            }
        }
    }

}


impl Clone for TcpServer {
    fn clone(&self) -> Self {
        Self {
            clients: self.clients.clone(),
            listener: self.listener.try_clone().expect("Failed to clone listener")
        }
    }
}


pub fn create_server(port:u64) -> std::io::Result<TcpServer> {
    TcpServer::new(format!("0.0.0.0:{}", port).as_str())
}
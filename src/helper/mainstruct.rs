use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value};
use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
pub struct CorePacket {
    pub message_uuid: Uuid,
    pub timestamp: u64,
    pub event_type: String,
    pub payload: Value,
    pub identifier_source: Uuid,
    pub identifier_destination: Uuid,
    pub hash: String,
    pub priority: u8,
    pub ttl: u64,
    pub consumed: bool,
}

impl CorePacket {
    pub fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_string(self).unwrap().as_bytes().to_vec()
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: Value) -> Self {
        let message_uuid = Uuid::new_v4();
        let timestamp = get_curent_timestamp();
        let event_type = json["event_type"].as_str().unwrap().to_string();
        let payload = json["payload"].clone();
        let identifier_source = Uuid::parse_str(json["identifier_source"].as_str().unwrap()).unwrap();
        let identifier_destination = Uuid::nil();
        let hash = "not-yet-implemented".to_string();
        let priority = json["priority"].as_u64().unwrap() as u8;
        let ttl = json["ttl"].as_u64().unwrap();
        let consumed = false;
        Self {
            message_uuid,
            timestamp,
            event_type,
            payload,
            identifier_source,
            identifier_destination,
            hash,
            priority,
            ttl,
            consumed,
        }
    }
}



fn get_curent_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_secs()
}
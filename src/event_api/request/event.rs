// External imports
use serde::Serialize;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Event {
    uuid: String,
    timestamp: u64,
    entity_id: String,
    #[serde(rename = "type")]
    event_type: String,
    event_key: Option<String>,
    // tags
}

impl Event {
    pub fn new(entity_id: String, event_type: String) -> Event {
        // Generate new UUID
        let uuid = Uuid::new_v4().as_hyphenated().to_string();

        // Get timestamp as milliseconds since the epoch
        let timestamp: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                // NOTE: Convert to u64 as json::object does not support u128
                duration.as_millis() as u64
            }
            Err(_) => 0,
        };

        Event {
            uuid,
            timestamp,
            entity_id,
            event_type,
            event_key: Option::None,
        }
    }
}

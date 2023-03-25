// External imports
use serde::Serialize;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Event {
    uuid: String,
    timestamp: u128,
    entity_id: String,
    event_key: String,
    // tags
}

impl Event {
    pub fn new(entity_id: String, event_key: String) -> Event {
        // Generate new UUID
        let uuid = Uuid::new_v4().as_hyphenated().to_string();

        // Get timestamp as milliseconds since the epoch
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_millis(),
            Err(_) => 0,
        };

        Event {
            uuid,
            timestamp,
            entity_id,
            event_key,
        }
    }
}

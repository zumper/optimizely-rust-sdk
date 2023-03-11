// External imports
use serde_json::{json, Value as JsonValue};
use std::time::SystemTime;
use uuid::Uuid;

pub struct Event {
    uuid: String,
    timestamp: u64,
    entity_id: String,
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

    pub fn as_json(self) -> JsonValue {
        // TODO: add these for custom events
        drop(self.event_key);

        json!({
            "entity_id": self.entity_id,
            "type": self.event_type,
            "timestamp": self.timestamp,
            "uuid": self.uuid,
        })
    }
}

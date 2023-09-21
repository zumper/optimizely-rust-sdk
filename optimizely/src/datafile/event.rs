// External imports
use error_stack::Result;

// Imports from super
use super::{Context, DatafileError};

#[derive(Debug)]
pub struct Event {
    id: String,
    key: String,
}

impl Event {
    // Create a new variation
    pub(crate) fn new<T: Into<String>>(id: T, key: T) -> Event {
        Event {
            id: id.into(),
            key: key.into(),
        }
    }

    /// Create a new variation from a Context
    pub(crate) fn build(context: &mut Context) -> Result<Event, DatafileError> {
        // Get variation_id as String
        let id = context.get("id")?.as_string()?;

        // Get variation_key as String
        let key = context.get("key")?.as_string()?;

        Ok(Event::new(id, key))
    }

    /// Getter for `id` field
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Getter for `key` field
    pub fn key(&self) -> &str {
        &self.key
    }
}

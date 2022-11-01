// External imports
use std::collections::HashMap;
// Imports from parent
use super::Decision;

// Custom type alias
pub type Attributes = HashMap<String, String>;

#[derive(Debug)]
pub struct UserContext {
    user_id: String,
    attributes: Attributes,
}

impl UserContext {
    pub fn new(user_id: &str) -> UserContext {
        // Create owned copies of user_id
        let user_id = user_id.to_owned();

        UserContext {
            user_id,
            attributes: HashMap::new(),
        }
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        // Create owned copies of the key and value
        let key = key.to_owned();
        let value = value.to_owned();

        // Add the attribute
        self.attributes.insert(key, value);
    }

    pub fn get_attributes(&self) -> &Attributes {
        // Return borrowed reference to attributes
        &self.attributes
    }

    pub fn decide(&self, flag_key: &str) -> Decision {
        // TODO: remove these two lines
        let _ = &self.user_id;
        drop(flag_key);

        Decision {}
    }
}

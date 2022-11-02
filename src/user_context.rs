// External imports
use std::collections::HashMap;
// Imports from parent
use super::DecideOption;
use super::Decision;

// Custom type alias
pub type UserAttributes = HashMap<String, String>;

// Inspiration: https://docs.developers.optimizely.com/experimentation/v4.0.0-full-stack/docs/optimizelyusercontext-python

#[derive(Debug)]
pub struct UserContext {
    user_id: String,
    attributes: UserAttributes,
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

    pub fn get_attributes(&self) -> &UserAttributes {
        // Return borrowed reference to attributes
        &self.attributes
    }

    pub fn decide<'a>(&'a self, flag_key: &'a str, _options: Vec<DecideOption>) -> Decision {
        // TODO: remove these two lines
        let _ = &self.user_id;
        drop(flag_key);

        Decision {
            flag_key,
            variation_key: "??",
            enabled: true,
        }
    }
}

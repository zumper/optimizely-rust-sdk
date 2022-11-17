// External imports
use std::collections::HashMap;
use std::rc::Rc;

// Imports from parent
use super::datafile::Datafile;
use super::decision::{DecideOption, Decision};

// Custom type alias
pub type UserAttributes = HashMap<String, String>;

// Inspiration: https://docs.developers.optimizely.com/experimentation/v4.0.0-full-stack/docs/optimizelyusercontext-python

#[derive(Debug)]
pub struct UserContext {
    datafile: Rc<Datafile>,
    user_id: String,
    attributes: UserAttributes,
}

impl UserContext {
    pub fn new(datafile: &Rc<Datafile>, user_id: &str) -> UserContext {
        // Create a clone of the reference, thus increasing the count
        let datafile = Rc::clone(&datafile);

        // Create owned copy of user_id
        let user_id = user_id.to_owned();

        // Create an empty set of user attributes
        let attributes = UserAttributes::new();

        UserContext {
            datafile,
            user_id,
            attributes,
        }
    }

    // TODO: add pub fn new_with_attributes

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

    pub fn decide<'a>(&'a self, flag_key: &'a str, _options: &Vec<DecideOption>) -> Decision {
        // Retrieve Flag object
        let flag = match self.datafile.get_flag(flag_key) {
            Some(flag) => flag,
            None => {
                // When flag key cannot be found, return the off variation
                // CONSIDERATION: Could have used Result<Decision, E> but this is how other Optimizely SDKs work
                return Decision::off(flag_key);
            }
        };

        // TODO: use Flag object
        drop(flag);

        match self.user_id.as_ref() {
            "user1" => Decision::new(flag_key, false, "off"),
            _ => Decision::new(flag_key, true, "on"),
        }
    }
}

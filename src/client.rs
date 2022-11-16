//! Everything related to make web requests to Optimizely

// External imports
use std::error::Error;
use std::rc::Rc;

// Imports from parent
use super::datafile::{Datafile, FeatureFlag};
use super::user_context::UserContext;

#[derive(Debug)]
pub struct Client {
    datafile: Rc<Datafile>,
}

impl Client {
    pub fn build(datafile: &str) -> Result<Client, Box<dyn Error>> {
        // Build datafile object from string
        let datafile = Datafile::build(datafile)?;
        // Create counted reference
        let datafile = Rc::new(datafile);

        // TODO: other properties of client

        Ok(Client { datafile })
    }

    pub fn account_id(&self) -> &str {
        &self.datafile.account_id()
    }

    pub fn revision(&self) -> u32 {
        self.datafile.revision()
    }

    pub fn feature_flags(&self) -> &Vec<FeatureFlag> {
        &self.datafile.feature_flags()
    }

    pub fn create_user_context(&self, user_id: &str) -> UserContext {
        UserContext::new(&self.datafile, user_id)
    }
}

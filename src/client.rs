//! Everything related to make web requests to Optimizely

// External imports
use anyhow::Result;
use std::fs::File;
use std::io::Read;

// Imports from parent
use super::datafile::{Datafile, FeatureFlag};
use super::user_context::UserContext;
use super::event::SimpleEventDispatcher;

// Relative imports of sub modules
pub use error::ClientError;

mod error;

#[derive(Debug)]
pub struct Client {
    pub(crate) datafile: Datafile,
    pub(crate) event_dispatcher: Box<SimpleEventDispatcher>, // TODO: use trait
}

impl Client {
    pub fn build_from_sdk_key(sdk_key: &str) -> Result<Client> {
        // Construct URL
        let url = format!("https://cdn.optimizely.com/datafiles/{}.json", sdk_key);

        // Make GET request
        // TODO: implement polling mechanism
        let response = ureq::get(&url)
            .call()
            .or_else(|_| Err(ClientError::FailedRequest))?;

        // Get response body
        let content = response
            .into_string()
            .or_else(|_| Err(ClientError::FailedResponse))?;

        // Use response to build Client
        Client::build_from_string(content)
    }

    pub fn build_from_file(file_path: &str) -> Result<Client> {
        // Read content from local path
        let mut content = String::new();

        // Open file
        let mut file = File::open(file_path).or_else(|_| Err(ClientError::FailedFileOpen))?;

        // Read file content into String
        file.read_to_string(&mut content)
            .or_else(|_| Err(ClientError::FailedFileRead))?;

        // Use file content to build Client
        Client::build_from_string(content)
    }

    pub fn build_from_string(content: String) -> Result<Client> {
        // Parse content as JSON
        let mut json_value = json::parse(&content)?;

        // Build datafile object from string
        let datafile = Datafile::build(&mut json_value)?;

        // Retrieve account ID from datafile
        let account_id = datafile.account_id().to_owned();

        // TODO: other properties of client
        let event_dispatcher = Box::new(SimpleEventDispatcher::new(account_id));

        Ok(Client { 
            datafile,
            event_dispatcher,
        })
    }

    pub fn account_id(&self) -> &str {
        &self.datafile.account_id()
    }

    pub fn revision(&self) -> u32 {
        self.datafile.revision()
    }

    pub fn feature_flags(&self) -> Vec<&FeatureFlag> {
        self.datafile.feature_flags()
    }

    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext {
        UserContext::new(self, user_id)
    }
}

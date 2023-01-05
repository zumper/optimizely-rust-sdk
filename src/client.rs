//! Everything related to make web requests to Optimizely

// External imports
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

// Imports from parent
use super::datafile::{Datafile, FeatureFlag};
use super::user_context::UserContext;

// Relative imports of sub modules
pub use error::ClientError;

mod error;

#[derive(Debug)]
pub struct Client {
    datafile: Rc<Datafile>,
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
        Client::build_from_string(&content)
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
        Client::build_from_string(&content)
    }

    pub fn build_from_string(content: &str) -> Result<Client> {
        // Parse content as JSON
        let mut json_value = json::parse(content)?;

        // Build datafile object from string
        let datafile = Datafile::build(&mut json_value)?;

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

    pub fn feature_flags(&self) -> Vec<&FeatureFlag> {
        self.datafile.feature_flags()
    }

    pub fn create_user_context(&self, user_id: &str) -> UserContext {
        UserContext::new(&self.datafile, user_id)
    }
}

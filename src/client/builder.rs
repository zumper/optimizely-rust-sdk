// External imports
use anyhow::Result;
use std::fs::File;
use std::io::Read;

// Imports from crate
use crate::client::{Client, ClientError};
use crate::datafile::Datafile;
use crate::event::{EventDispatcher, SimpleEventDispatcher};

pub struct ClientBuilder {
    datafile: Option<Datafile>,
    event_dispatcher: Option<Box<dyn EventDispatcher>>,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            datafile: None,
            event_dispatcher: None,
        }
    }

    pub fn with_sdk_key(mut self, sdk_key: &str) -> Result<ClientBuilder> {
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
        self.datafile = Some(Datafile::build(content)?);

        Ok(self)
    }

    pub fn with_local_datafile(mut self, file_path: &str) -> Result<ClientBuilder> {
        // Read content from local path
        let mut content = String::new();

        // Open file
        let mut file = File::open(file_path).or_else(|_| Err(ClientError::FailedFileOpen))?;

        // Read file content into String
        file.read_to_string(&mut content)
            .or_else(|_| Err(ClientError::FailedFileRead))?;

        // Use file content to build Client
        self.datafile = Some(Datafile::build(content)?);

        Ok(self)
    }

    pub fn with_datafile_as_string(mut self, content: String) -> Result<ClientBuilder> {
        self.datafile = Some(Datafile::build(content)?);
        Ok(self)
    }

    pub fn with_event_dispatcher(mut self, event_dispatcher: impl EventDispatcher + 'static) -> ClientBuilder {
        self.event_dispatcher = Some(Box::new(event_dispatcher));
        self
    }

    pub fn build(self) -> Result<Client> {
        let datafile = self.datafile.ok_or(ClientError::DatafileMissing)?;
        let event_dispatcher = self
            .event_dispatcher
            .unwrap_or_else(|| Box::new(SimpleEventDispatcher::new()));

        Ok(Client {
            datafile,
            event_dispatcher,
        })
    }
}

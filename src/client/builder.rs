// External imports
use anyhow::Result;

// Imports from crate
use crate::client::{Client, ClientError};
use crate::event::{EventDispatcher, SimpleEventDispatcher};
use crate::Datafile;

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

    pub fn with_datafile(mut self, datafile: Datafile) -> ClientBuilder {
        self.datafile = Some(datafile);
        self
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

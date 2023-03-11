//! Everything related to make web requests to Optimizely

// Imports from crate
use crate::UserContext;
use crate::datafile::Datafile;
use crate::event::EventDispatcher;

// Relative imports of sub modules
pub use error::ClientError;
pub use builder::ClientBuilder;

mod error;
mod builder;


pub struct Client {
    datafile: Datafile,
    event_dispatcher: Box<dyn EventDispatcher>,
}

impl Client {
    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext {
        UserContext::new(self, user_id)
    }

    /// Getter for `account_id` field of `datafile`
    pub fn account_id(&self) -> &str {
        self.datafile.account_id()
    }

    /// Getter for `revision` field of `datafile`
    pub fn revision(&self) -> u32 {
        self.datafile.revision()
    }

    /// Getter for `datafile` field
    pub(crate) fn datafile(&self) -> &Datafile {
        &self.datafile
    }

    /// Getter for `event_dispatcher` field
    pub(crate) fn event_dispatcher(&self) -> &Box<dyn EventDispatcher> {
        &self.event_dispatcher
    }
}

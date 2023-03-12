//! Entrypoint of the SDK

// Imports from crate
use crate::datafile::Datafile;
use crate::event::EventDispatcher;

// Relative imports of sub modules
pub use user::{UserContext, UserAttributes};
pub use error::ClientError;
pub use builder::ClientBuilder;

mod error;
mod builder;
mod user;

/// SDK client to use Optimizely Feature Experimentation
pub struct Client {
    datafile: Datafile,
    event_dispatcher: Box<dyn EventDispatcher>,
}

impl Client {

    /// Create a new user context for a given user id
    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext {
        UserContext::new(self, user_id)
    }

    /// Get the current Optimizely account id
    pub fn account_id(&self) -> &str {
        self.datafile.account_id()
    }

    /// Get the current revision of the datafile
    pub fn revision(&self) -> u32 {
        self.datafile.revision()
    }

    /// Get the datafile within the client
    pub(crate) fn datafile(&self) -> &Datafile {
        &self.datafile
    }

    /// Get the event dispatcher within the client
    pub(crate) fn event_dispatcher(&self) -> &Box<dyn EventDispatcher> {
        &self.event_dispatcher
    }
}

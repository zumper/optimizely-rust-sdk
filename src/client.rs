//! Everything related to make web requests to Optimizely

// Imports from parent
use super::datafile::Datafile;
use super::user_context::UserContext;
use super::event::{SimpleEventDispatcher, EventDispatcher};

pub struct Client {
    pub(crate) datafile: Datafile,
    pub(crate) event_dispatcher: Box<dyn EventDispatcher>,
}

impl Client {
    // TODO: create several constructors with different options
    pub fn new(datafile: Datafile) -> Client {
        let account_id = datafile.account_id().to_owned();
        let event_dispatcher = SimpleEventDispatcher::new(account_id);

        Client {
            datafile,
            event_dispatcher: Box::new(event_dispatcher),
        }
    }

    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext {
        UserContext::new(self, user_id)
    }
}

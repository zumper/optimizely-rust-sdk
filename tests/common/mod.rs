// Incorrect warnings of dead code: https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

// External imports
use std::cell::RefCell;
use std::rc::Rc;

// Imports from Optimizely crate
use optimizely::client::{Client, ClientBuilder};
use optimizely::event_api::{Event, EventDispatcher};

// This is the account ID of mark.biesheuvel@optimizely.com
pub const ACCOUNT_ID: &str = "21537940595";

// SDK key for the development environment of mark.biesheuvel@optimizely.com
// This key only grants read access to a JSON file and does not grant any further permissions
pub const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";

// This is a bundled copy of the JSON file that can be downloaded with the SDK key
pub const FILE_PATH: &str = "examples/datafiles/sandbox.json";

// This is the revision number of the bundled datafile
pub const REVISION: u32 = 73;

// List of Events wrapped in a reference counted mutable memory location
type EventList = Rc<RefCell<Vec<Event>>>;

// Struct that holds the EventList and implement the EventDispatcher trait
#[derive(Default)]
pub(super) struct EventStore {
    list: Rc<RefCell<Vec<Event>>>,
}

// Return a new reference counted point to the list
impl EventStore {
    fn list(&self) -> Rc<RefCell<Vec<Event>>> {
        Rc::clone(&self.list)
    }
}

// Implementing the EventDispatcher using the interior mutability pattern
impl EventDispatcher for EventStore {
    fn send_event(&self, event: Event) {
        self.list.borrow_mut().push(event);
    }
}

// Return struct from setup function that contains:
// - an Optimizely client
// - a list of events that was send to the EventDispatcher
pub struct TestContext {
    pub client: Client,
    pub event_list: EventList,
}

// A setup function used in mutliple tests
pub(super) fn setup() -> TestContext {
    // Create a struct to store events
    let event_store = EventStore::default();
    let event_list = event_store.list();

    // Build client
    let client = ClientBuilder::new()
        .with_event_dispatcher(event_store)
        .with_local_datafile(FILE_PATH)
        .expect("local datafile should work")
        .build();

    TestContext { client, event_list }
}

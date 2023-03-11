// Incorrect warnings of dead code: https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

// Imports from this crate
use optimizely::{Client, ClientBuilder, DecideOptions};

// This is the account ID of mark.biesheuvel@optimizely.com
pub const ACCOUNT_ID: &str = "21537940595";

// SDK key for the development environment of mark.biesheuvel@optimizely.com
// This key only grants read access to a JSON file and does not grant any further permissions
pub const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";

// This is a bundled copy of the JSON file that can be downloaded with the SDK key
pub const FILE_PATH: &str = "examples/datafiles/sandbox.json";

// This is the revision number of the bundled datafile
pub const REVISION: u32 = 73;

pub(super) struct TestContext {
    pub(super) client: Client,
    pub(super) decide_options: DecideOptions,
}

pub(super) fn setup() -> TestContext {
    // Build a client using default settings
    let client = ClientBuilder::new()
        .with_local_datafile(FILE_PATH)
        .expect("local datafile should work")
        .build()
        .expect("build should work");

    // Do not send any decision events during testing
    let decide_options = DecideOptions {
        disable_decision_event: true,
        ..DecideOptions::default()
    };

    TestContext { client, decide_options }
}

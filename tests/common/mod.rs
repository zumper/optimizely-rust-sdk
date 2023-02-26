// Incorrect warnings of dead code: https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

// Imports from this crate
use optimizely::{Client, Datafile, DecideOption, Decision};

// This is the account ID of mark.biesheuvel@optimizely.com
pub const ACCOUNT_ID: &str = "21537940595";

// SDK key for the development environment of mark.biesheuvel@optimizely.com
// This key only grants read access to a JSON file and does not grant any further permissions
pub const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";

// This is a bundled copy of the JSON file that can be downloaded with the SDK key
pub const FILE_PATH: &str = "examples/datafiles/sandbox.json";

// This is the revision number of the bundled datafile
pub const REVISION: u32 = 73;

// Helper function create a fixed client
pub fn get_client() -> Client {
    let datafile = Datafile::build_from_file(FILE_PATH).expect("local file should work");
    Client::new(datafile)
}

pub fn get_decision<'a, 'b>(user_id: &'b str, flag_key: &'a str) -> Decision<'a> {
    let client = get_client();
    let user_context = client.create_user_context(user_id);

    // Do not send any decision events during testing
    let decide_options = vec![DecideOption::DisableDecisionEvent];

    // Return decision result
    user_context.decide(flag_key, &decide_options)
}

// External imports
use std::fs::File;
use std::io::Read;
// Imports from this crate
use optimizely::attributes;
use optimizely::{Attributes, Optimizely};

const FILE_PATH: &str = "tests/datafile.example.json";

fn client() -> Optimizely {
    // Read datafile from local path
    let mut datafile = String::new();
    File::open(FILE_PATH)
        .expect("should be able to open file")
        .read_to_string(&mut datafile)
        .expect("should be able to read to string");

    // Example datafile is valid
    Optimizely::build(&datafile).expect("should be a valid datafile")
}

#[test]
fn empty_datafile() {
    // Empty datafile is invalid
    let result = Optimizely::build(r"");
    matches!(result, Err(_));
}

#[test]
fn client_initialization() {
    let client = client();

    // Check property on client
    assert!(client.initialized);
}

#[test]
fn create_user_context() {
    let client = client();

    // Create a user context
    let user_id = "user123";
    let mut user_context = client.create_user_context(user_id);

    user_context.set_attribute("app_version", "1.3.2");

    let expected = attributes! {
        "app_version" => "1.3.2"
    };

    //
    assert_eq!(*user_context.get_attributes(), expected);
}

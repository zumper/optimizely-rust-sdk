// External imports
use std::fs::File;
use std::io::Read;
// Imports from this crate
use optimizely::user_attributes;
use optimizely::{Client, UserAttributes};

const FILE_PATH: &str = "tests/datafile.example.json";

fn client() -> Client {
    // Read datafile from local path
    let mut datafile = String::new();
    File::open(FILE_PATH)
        .expect("should be able to open file")
        .read_to_string(&mut datafile)
        .expect("should be able to read to string");

    // Example datafile is valid
    Client::build(&datafile).expect("should be a valid datafile")
}

#[test]
fn empty_datafile() {
    // Empty datafile is invalid
    let result = Client::build(r"");
    matches!(result, Err(_));
}

#[test]
fn client_initialization() {
    let client = client();

    // Check property on client
    assert_eq!(client.account_id, "21537940595");
    assert_eq!(client.revision, 73);
}

#[test]
fn user_context_set_attribute_method() {
    let client = client();
    let mut user_context = client.create_user_context("user123");

    user_context.set_attribute("is_employee", "true");
    user_context.set_attribute("app_version", "1.3.2");

    // TODO: verify whether non-existing attributes should be returned
    let expected = user_attributes! {
        "is_employee" => "true",
        "app_version" => "1.3.2",
    };

    // Attributes should be equal to expected
    assert_eq!(*user_context.get_attributes(), expected);
}

#[test]
fn user_context_decide_method() {
    let client = client();
    let user_context = client.create_user_context("user123");

    let flag_key = "qa_rollout";

    let decision = user_context.decide(flag_key, Vec::new());

    // TODO: assert_eq variation_key
    assert!(decision.enabled);
    assert_eq!(decision.flag_key, flag_key);
}

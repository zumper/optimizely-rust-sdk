// External imports
use std::fs::File;
use std::io::Read;
// Imports from this crate
use optimizely::client::Client;
use optimizely::user_attributes;
use optimizely::user_context::UserAttributes;

const FILE_PATH: &str = "examples/datafile.json";

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
    assert_eq!(client.account_id(), "21537940595");
    assert_eq!(client.revision(), 73);

    let flags = client.feature_flags();

    // Check if flags are there
    assert_eq!(flags.len(), 6);
    assert!(flags.iter().any(|flag| flag.key == "buy_button"));
    assert!(flags.iter().any(|flag| flag.key == "qa_rollout"));
}

#[test]
fn user_context_attributes() {
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
fn decision_qa_rollout_flag() {
    let client = client();

    let flag_key = "qa_rollout";
    let decide_options = Vec::new();

    // The flag should be off for `user1 `
    let user_context = client.create_user_context("user1");
    let decision = user_context.decide(flag_key, &decide_options);
    assert!(!decision.enabled());
    assert_eq!(decision.variation_key(), "off");

    // The flag should be on for `user3 `
    let user_context = client.create_user_context("user3");
    let decision = user_context.decide(flag_key, &decide_options);
    assert!(decision.enabled());
    assert_eq!(decision.variation_key(), "on");
}

#[test]
fn decision_invalid_flag() {
    let client = client();

    let flag_key = "this_flag_does_not_exist";
    let decide_options = Vec::new();

    // An invalid flag should always be disabled
    let user_context = client.create_user_context("_");
    let decision = user_context.decide(flag_key, &decide_options);
    assert!(!decision.enabled());
    assert_eq!(decision.variation_key(), "off");
}
// Imports from this crate
use optimizely::client::Client;

// Relative imports of sub modules
use common::{ACCOUNT_ID, FILE_PATH, REVISION, SDK_KEY};
mod common;

#[test]
fn with_empty_datafile() {
    // Empty datafile is invalid
    let result = Client::build_from_string(r"");
    matches!(result, Err(_));
}

#[test]
fn with_sdk_key() {
    let result = Client::build_from_sdk_key(SDK_KEY);

    // Check whether client successfully initialized
    matches!(result, Ok(_));

    if let Ok(client) = result {
        // Check property on client
        assert_eq!(client.account_id(), ACCOUNT_ID);
        // The online datafile might have been updated
        assert!(client.revision() >= REVISION);
    }
}

#[test]
fn with_fixed_datafile() {
    let result = Client::build_from_file(FILE_PATH);

    // Check whether client successfully initialized
    matches!(result, Ok(_));

    if let Ok(client) = result {
        // Check property on client
        assert_eq!(client.account_id(), ACCOUNT_ID);
        assert_eq!(client.revision(), REVISION);

        let flags = client.feature_flags();

        // Check if flags are there
        assert_eq!(flags.len(), 6);
        assert!(flags.iter().any(|flag| flag.key() == "buy_button"));
        assert!(flags.iter().any(|flag| flag.key() == "qa_rollout"));
    }
}

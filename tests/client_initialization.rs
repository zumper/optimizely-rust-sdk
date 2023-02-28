// Imports from this crate
use optimizely::ClientBuilder;

// Relative imports of sub modules
use common::{ACCOUNT_ID, FILE_PATH, REVISION, SDK_KEY};
mod common;

#[test]
fn with_empty_datafile() {
    // Empty datafile is invalid
    let empty_string = "".to_owned();
    let result = ClientBuilder::new().with_datafile_as_string(empty_string);
    matches!(result, Err(_));
}

#[test]
fn with_sdk_key() {
    let result = ClientBuilder::new().with_sdk_key(SDK_KEY);

    // Check whether datafile successfully initialized
    matches!(result, Ok(_));

    if let Ok(client_builder) = result {
        let client = client_builder.build().expect("build should work");

        // Check property on client
        assert_eq!(client.account_id(), ACCOUNT_ID);
        // The online datafile might have been updated
        assert!(client.revision() >= REVISION);
    }
}

#[test]
fn with_fixed_datafile() {
    let result = ClientBuilder::new().with_local_datafile(FILE_PATH);

    // Check whether client successfully initialized
    matches!(result, Ok(_));

    if let Ok(client_builder) = result {
        let client = client_builder.build().expect("build should work");

        // Check property on client
        assert_eq!(client.account_id(), ACCOUNT_ID);
        assert_eq!(client.revision(), REVISION);

        // let flags = datafile.feature_flags();

        // // Check if flags are there
        // assert_eq!(flags.len(), 6);
        // assert!(flags.iter().any(|flag| flag.key() == "buy_button"));
        // assert!(flags.iter().any(|flag| flag.key() == "qa_rollout"));
    }
}

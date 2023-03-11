// Imports from this crate
use optimizely::{DatafileError, ClientBuilder, ClientError};

// Relative imports of sub modules
use common::{ACCOUNT_ID, FILE_PATH, REVISION, SDK_KEY};
mod common;

#[test]
fn with_invalid_json() {
    // Empty datafile is invalid
    let json = "";

    // Get error report
    let report = ClientBuilder::new()
        .with_datafile_as_string(json.into())
        .err()
        .unwrap();

    // Verify the client error type
    let client_error = report.downcast_ref::<ClientError>().unwrap();
    assert!(matches!(client_error, ClientError::InvalidDatafile));

    // Verify the datafile error type
    let datafile_error = report.downcast_ref::<DatafileError>().unwrap();
    assert!(matches!(datafile_error, DatafileError::InvalidJson));
}

#[test]
fn with_missing_properties() {
    // Valid JSON, but missing properties
    let json = r#"
    {
        "accountId": "21537940595",
        "revision": "73"
    }"#;

    // Get error report
    let report = ClientBuilder::new()
        .with_datafile_as_string(json.into())
        .err()
        .unwrap();

    // Verify the client error type
    let client_error = report.downcast_ref::<ClientError>().unwrap();
    assert!(matches!(client_error, ClientError::InvalidDatafile));

    // Verify the datafile error type
    let datafile_error = report.downcast_ref::<DatafileError>().unwrap();
    assert!(matches!(datafile_error, DatafileError::MissingField(_)));
}

#[test]
fn with_invalid_array_propertie() {
    // Valid JSON, but rollouts, experiments, and featureFlags should be an array
    let json = r#"
    {
        "accountId": "21537940595",
        "revision": "73",
        "rollouts": null,
        "experiments": null,
        "featureFlags": null
    }"#;

    // Get error report
    let report = ClientBuilder::new()
        .with_datafile_as_string(json.into())
        .err()
        .unwrap();

    // Verify the client error type
    let client_error = report.downcast_ref::<ClientError>().unwrap();
    assert!(matches!(client_error, ClientError::InvalidDatafile));

    // Verify the datafile error type
    let datafile_error = report.downcast_ref::<DatafileError>().unwrap();
    assert!(matches!(datafile_error, DatafileError::MissingField(_)));
}

#[test]
fn with_sdk_key() {
    let client = ClientBuilder::new()
        .with_sdk_key(SDK_KEY)
        .expect("sdk key shoudl work")
        .build()
        .expect("build should work");

    // Check account id property on client
    assert_eq!(client.account_id(), ACCOUNT_ID);

    // Check revision property on client
    // NOTE: the online datafile might have been updated
    assert!(client.revision() >= REVISION);
}

#[test]
fn with_fixed_datafile() {
    let client = ClientBuilder::new()
        .with_local_datafile(FILE_PATH)
        .expect("local datafile should work")
        .build()
        .expect("build should work");

    // Check account id property on client
    assert_eq!(client.account_id(), ACCOUNT_ID);

    // Check revision property on client
    assert_eq!(client.revision(), REVISION);
}

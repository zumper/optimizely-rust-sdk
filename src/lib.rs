#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

// Reimport/export of structs to make them available at top-level
pub use client::ClientBuilder;

// Regular modules
pub mod client;
pub mod datafile;
pub mod decision;
pub mod event;

/// Used in doctest
pub fn doctest_client() -> error_stack::Result<client::Client, client::ClientError> {
    let sdk_key = "KVpGWnzPGKvvQ8yeEWmJZ";
    let client = ClientBuilder::new()
        .with_sdk_key(sdk_key)?
        .build()?;

    Ok(client)
}

#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

// Reimport/export of structs to make them available at top-level
pub use client::Client;

// Regular modules
pub mod client;
pub mod datafile;
pub mod decision;

#[cfg(feature = "online")]
pub mod event_api;

//! Optimizely Feature Experimentation SDK

// Reimport/export of structs to make them available at top-level
pub use client::{Client, ClientBuilder, ClientError};
pub use decision::{DecideOptions, Decision};
pub use user_context::{UserAttributes, UserContext};
pub use datafile::DatafileError;

// Regular modules
mod client;
pub mod datafile;
mod decision;
mod user_context;
pub mod event;

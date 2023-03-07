//! Optimizely Feature Experimentation SDK

// Macro module
#[macro_use]
mod macros;

// Reimport/export of structs to make them available at top-level
pub use client::{Client, ClientBuilder};
pub use decision::{DecideOptions, Decision};
pub use user_context::{UserAttributes, UserContext};

// Regular modules
mod client;
pub mod datafile;
mod decision;
mod user_context;
pub mod event;

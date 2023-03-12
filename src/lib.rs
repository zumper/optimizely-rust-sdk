//! # Optimizely Feature Experimentation SDK
//! 
//! An **unofficial** Rust SDK for Optimizely Feature Experimentation.
//! 
//! This SDK is **not** supported by Optimizely!
//! 
//! This SDK only includes a small subset of features compared to supported SDKs. Use at own risk!
//! 
//! ## Example
//! ```
//! // Initialize Optimizely client using SDK key
//! let sdk_key = "KVpGWnzPGKvvQ8yeEWmJZ";
//! let client = optimizely::ClientBuilder::new()
//!     .with_sdk_key(sdk_key).unwrap()
//!     .build().unwrap();
//! 
//! // Create user context for current user
//! let user_id = "123abc789xyz";
//! let user_context = client.create_user_context(user_id);
//! 
//! // Get decision for the Buy Button feature flag
//! let feature_flag = "buy_button";
//! let decision = user_context.decide(feature_flag);
//! ```

// Reimport/export of structs to make them available at top-level
pub use client::ClientBuilder;

// Regular modules
pub mod client;
pub mod datafile;
pub mod decision;
pub mod user_context;
pub mod event;

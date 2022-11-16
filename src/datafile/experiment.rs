// External imports
use json::JsonValue;
// Imports from parent
use super::DatafileError;

/// Optimizely experiment
#[derive(Debug)]
pub struct Experiment {
    pub key: String,
}

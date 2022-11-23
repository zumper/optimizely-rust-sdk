// External imports
use anyhow::Result;
use json::JsonValue;

// Imports from parent
use super::DatafileError;

/// Optimizely experiment
#[derive(Debug)]
pub struct Experiment {
    pub key: String,
    pub status: String,
}

impl Experiment {
    pub fn build(value: &mut JsonValue) -> Result<Experiment> {
        let key = string_field!(value, "key")?;
        let status = string_field!(value, "status")?;

        let experiment = Experiment { key, status };
        Ok(experiment)
    }
}

// External imports
use anyhow::Result;
use json::JsonValue;
use std::collections::HashMap;

// Imports from parent
use super::{DatafileError, Variation};

/// Optimizely experiment
#[derive(Debug)]
pub struct Experiment {
    pub id: String,
    pub key: String,
    pub status: String,
    pub variations: HashMap<String, Variation>,
}

impl Experiment {
    pub fn build(datafile: &mut JsonValue) -> Result<Experiment> {
        let id = string_field!(datafile, "id")?;
        let key = string_field!(datafile, "key")?;
        let status = string_field!(datafile, "status")?;

        let variations: Vec<Variation> = list_field!(datafile, "variations", Variation::build)?;
        let variations: HashMap<String, Variation> = list_to_map!(variations, Variation::map_entry);

        let experiment = Experiment {
            id,
            key,
            status,
            variations,
        };
        Ok(experiment)
    }

    pub fn map_entry(self) -> (String, Experiment) {
        (self.id.clone(), self)
    }
}

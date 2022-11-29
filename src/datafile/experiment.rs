// External imports
use anyhow::Result;
use json::JsonValue;
use std::collections::HashMap;

// Imports from parent
use super::{TrafficAllocation, Variation};

/// Optimizely experiment
#[derive(Debug)]
pub struct Experiment {
    pub id: String,
    pub key: String,
    pub status: String,
    pub traffic_allocation: TrafficAllocation,
}

impl Experiment {
    pub fn build(datafile: &mut JsonValue) -> Result<Experiment> {
        // Get fields as string
        let id = string_field!(datafile, "id")?;
        let key = string_field!(datafile, "key")?;
        let status = string_field!(datafile, "status")?;

        // Create map of all variation so they can be looked up within TrafficAllocation
        let variations: Vec<Variation> = list_field!(datafile, "variations", Variation::build)?;
        let mut variations: HashMap<String, Variation> = list_to_map!(variations, Variation::map_entry);

        // Build TrafficAllocation struct
        let traffic_allocation = TrafficAllocation::build(datafile, &mut variations)?;

        // Initialize struct and return result
        let experiment = Experiment {
            id,
            key,
            status,
            traffic_allocation,
        };
        Ok(experiment)
    }

    pub fn map_entry(self) -> (String, Experiment) {
        (self.id.clone(), self)
    }
}

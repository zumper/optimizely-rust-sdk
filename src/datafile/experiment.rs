// External imports
use anyhow::Result;
use json::JsonValue;

// Imports from crate
use crate::datafile::{TrafficAllocation, Variation};

/// Optimizely experiment
#[derive(Debug)]
pub struct Experiment {
    id: String,
    campaign_id: String,
    traffic_allocation: TrafficAllocation,
}

impl Default for Experiment {
    fn default() -> Self {
        Experiment {
            id: "".to_owned(),
            campaign_id: "".to_owned(),
            traffic_allocation: TrafficAllocation::default(),
        }
    }
}

impl Experiment {
    pub fn build(value: &mut JsonValue) -> Result<Experiment> {
        // Get fields as string
        let id = string_field!(value, "id")?;
        let _key = string_field!(value, "key")?;
        let campaign_id = string_field!(value, "layerId")?;
        let _status = string_field!(value, "status")?;

        // TODO: handle different values for status

        // Create map of all variation so they can be looked up within TrafficAllocation
        let variations = list_field!(value, "variations", Variation::build)?;
        let mut variations = Variation::list_to_map(variations);

        // Build TrafficAllocation struct
        let traffic_allocation = TrafficAllocation::build(value, &mut variations)?;

        // Initialize struct and return result
        let experiment = Experiment {
            id,
            campaign_id,
            traffic_allocation,
        };
        Ok(experiment)
    }

    pub fn map_entry(self) -> (String, Experiment) {
        (self.id.clone(), self)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn campaign_id(&self) -> &str {
        &self.campaign_id
    }

    pub fn traffic_allocation(&self) -> &TrafficAllocation {
        &self.traffic_allocation
    }
}

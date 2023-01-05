// External imports
use anyhow::Result;
use json::JsonValue;
use std::collections::HashMap;
use std::rc::Rc;

// Imports from parent
use super::{TrafficAllocation, Variation};

/// Optimizely experiment
#[derive(Debug)]
pub struct Experiment {
    id: String,
    campaign_id: String,
    traffic_allocation: TrafficAllocation,
}

impl Experiment {
    pub fn build(datafile: &mut JsonValue) -> Result<Experiment> {
        // Get fields as string
        let id = string_field!(datafile, "id")?;
        let _key = string_field!(datafile, "key")?;
        let campaign_id = string_field!(datafile, "layerId")?;
        let _status = string_field!(datafile, "status")?;

        // TODO: handle different values for status

        // Create map of all variation so they can be looked up within TrafficAllocation
        let variations: Vec<Variation> = list_field!(datafile, "variations", Variation::build)?;
        let mut variations: HashMap<String, Rc<Variation>> = list_to_map!(variations, Variation::map_entry);

        // Build TrafficAllocation struct
        let traffic_allocation = TrafficAllocation::build(datafile, &mut variations)?;

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

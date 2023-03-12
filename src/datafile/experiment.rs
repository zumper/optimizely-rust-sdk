// External imports
use error_stack::Result;
use std::collections::HashMap;
use std::rc::Rc;

// Imports from super
use super::{DatafileError, Json, TrafficAllocation, Variation};

/// Optimizely experiment
#[derive(Debug, Default)]
pub struct Experiment {
    id: String,
    campaign_id: String,
    traffic_allocation: TrafficAllocation,
}

impl Experiment {
    pub(crate) fn build(json: &mut Json) -> Result<Experiment, DatafileError> {
        // Get fields as string
        let id = json.get("id")?.as_string()?;

        let campaign_id = json.get("layerId")?.as_string()?;

        // TODO: retrieve key
        // TODO: retrieve status and handle different values for status

        // Create map of all variation so they can be looked up within TrafficAllocation
        let mut variations = json
            .get("variations")?
            .as_array()?
            .map(|mut json| Variation::build(&mut json))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|variation| (variation.id().to_owned(), Rc::new(variation)))
            .collect::<HashMap<_, _>>();

        // Build TrafficAllocation struct
        let traffic_allocation = TrafficAllocation::build(json, &mut variations)?;

        Ok(Experiment {
            id,
            campaign_id,
            traffic_allocation,
        })
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

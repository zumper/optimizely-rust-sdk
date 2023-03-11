// External imports
use error_stack::{IntoReport, Result};
use serde_json::Value as JsonValue;

// Imports from crate
use crate::datafile::{DatafileError, Experiment};

#[derive(Debug)]
pub struct Rollout {
    id: String,
    experiments: Vec<Experiment>,
}

impl Rollout {
    pub fn build(value: &mut JsonValue) -> Result<Rollout, DatafileError> {
        let id = string_field!(value, "id");

        let experiments = list_field!(value, "experiments", Experiment::build);

        Ok(Rollout {
            id: id.to_owned(),
            experiments,
        })
    }

    pub fn map_entry(self) -> (String, Rollout) {
        (self.id.clone(), self)
    }

    pub fn experiments(&self) -> &Vec<Experiment> {
        &self.experiments
    }
}

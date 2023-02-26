// External imports
use anyhow::Result;
use json::JsonValue;

// Imports from crate
use crate::datafile::Experiment;

#[derive(Debug)]
pub struct Rollout {
    id: String,
    experiments: Vec<Experiment>,
}

impl Rollout {
    pub fn build(value: &mut JsonValue) -> Result<Rollout> {
        let id = string_field!(value, "id")?;

        let experiments = list_field!(value, "experiments", Experiment::build)?;

        let rollout = Rollout { id, experiments };
        Ok(rollout)
    }

    pub fn map_entry(self) -> (String, Rollout) {
        (self.id.clone(), self)
    }

    pub fn experiments(&self) -> &Vec<Experiment> {
        &self.experiments
    }
}

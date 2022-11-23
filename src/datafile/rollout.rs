// External imports
use anyhow::Result;
use json::JsonValue;

// Imports from parent
use super::{DatafileError, Experiment};

#[derive(Debug)]
pub struct Rollout {
    pub id: String,
    pub experiments: Vec<Experiment>,
}

impl Rollout {
    pub fn build(datafile: &mut JsonValue) -> Result<Rollout> {
        let id = string_field!(datafile, "id")?;

        let experiments = list_field!(datafile, "experiments", Experiment::build)?;

        let rollout = Rollout { id, experiments };
        Ok(rollout)
    }

    pub fn map_entry(self) -> (String, Rollout) {
        (self.id.clone(), self)
    }
}

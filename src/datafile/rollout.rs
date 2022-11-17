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
    pub fn build(value: &mut JsonValue) -> Result<Rollout> {
        let id = string_field!(value, "id")?;

        let experiment_closure = |value| Experiment::build(value);
        let experiments = list_field!(value, "experiments", experiment_closure)?;

        let rollout = Rollout { id, experiments };
        Ok(rollout)
    }
}

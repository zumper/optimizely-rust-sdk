// External imports
use json::JsonValue;
// Imports from parent
use super::DatafileError;

#[derive(Debug)]
pub struct Rollout {
    pub id: String,
}

impl Rollout {
    pub fn build(value: &mut JsonValue) -> Result<Rollout, DatafileError> {
        let id = string_field!(value, "id")?;

        let experiments = list_field!(value, "experiments", |_| Ok(()))?;

        dbg!(experiments.len());

        let rollout = Rollout { id };
        Ok(rollout)
    }
}

// External imports
use json::JsonValue;
// Imports from parent
use super::DatafileError;

#[derive(Debug)]
pub struct FeatureFlag {
    pub id: String,
    pub key: String,
    // rollout: Rollout,
}

impl FeatureFlag {
    pub fn build(value: &mut JsonValue) -> Result<FeatureFlag, DatafileError<'static>> {
        let id = value["id"]
            .take_string()
            .ok_or(DatafileError::MissingField("id"))?;

        let key = value["key"]
            .take_string()
            .ok_or(DatafileError::MissingField("key"))?;

        Ok(FeatureFlag { id, key })
    }
}

// External imports
use anyhow::Result;
use json::JsonValue;

// Imports from parent
use super::DatafileError;

#[derive(Debug)]
pub struct Variation {
    pub id: String,
    pub key: String,
    pub is_feature_enabled: bool,
}

impl Variation {
    pub fn build(datafile: &mut JsonValue) -> Result<Variation> {
        let id = string_field!(datafile, "id")?;
        let key = string_field!(datafile, "key")?;
        let is_feature_enabled = bool_field!(datafile, "featureEnabled")?;

        let variation = Variation {
            id,
            key,
            is_feature_enabled,
        };
        Ok(variation)
    }

    pub fn map_entry(self) -> (String, Variation) {
        (self.id.clone(), self)
    }
}

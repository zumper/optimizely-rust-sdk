// External imports
use anyhow::Result;
use json::JsonValue;
use std::rc::Rc;

#[derive(Debug)]
pub struct Variation {
    id: String,
    key: String,
    is_feature_enabled: bool,
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

    pub fn map_entry(self) -> (String, Rc<Variation>) {
        (self.id.clone(), Rc::new(self))
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn is_feature_enabled(&self) -> bool {
        self.is_feature_enabled
    }
}

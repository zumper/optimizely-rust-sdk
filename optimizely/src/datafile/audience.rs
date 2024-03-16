// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// Imports from super
use super::{AudienceCondition, BooleanCondition};

#[derive(Deserialize, Debug)]
pub struct Audience {
    conditions: BooleanCondition<AudienceCondition>,
    id: String,
    name: String,
}

impl Audience {
    // Method to deserialize an array of Audiences into a Hashmap of Audiences
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Audience>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for audience in Vec::<Audience>::deserialize(deserializer)? {
            map.insert(audience.id.clone(), audience);
        }
        Ok(map)
    }

    /// Getter for `id` field
    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Getter for `name` field
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Getter for `conditions` field
    #[allow(dead_code)]
    pub fn conditions(&self) -> &BooleanCondition<AudienceCondition> {
        &self.conditions
    }
}

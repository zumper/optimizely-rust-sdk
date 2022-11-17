// External imports
use anyhow::Result;
use json::JsonValue;
use std::collections::HashMap;

// Imports from parent
use super::{DatafileError, Rollout};

/// Optimizely feature flag.
#[derive(Debug)]
pub struct FeatureFlag {
    pub id: String,
    pub key: String,
    pub rollout: Rollout,
    // TODO: variables
    // TODO: experiments
}

impl FeatureFlag {
    /// Builds a feature flag from JSON datafile
    pub fn build(
        value: &mut JsonValue,
        rollout_map: &mut HashMap<String, Rollout>,
    ) -> Result<FeatureFlag> {
        let id = string_field!(value, "id")?;
        let key = string_field!(value, "key")?;
        let rollout_id = string_field!(value, "rolloutId")?;

        // Remove from hashmap to get an owned copy
        let rollout = rollout_map
            .remove(&rollout_id)
            .ok_or(DatafileError::InvalidRolloutId(rollout_id))?;

        let flag = FeatureFlag { id, key, rollout };
        Ok(flag)
    }
}

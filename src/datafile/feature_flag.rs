// External imports
use anyhow::Result;
use json::JsonValue;
use std::collections::HashMap;

// Imports from parent
use super::{DatafileError, Experiment, Rollout};

/// Optimizely feature flag.
#[derive(Debug)]
pub struct FeatureFlag {
    key: String,
    rollout: Rollout,
    experiments: Vec<Experiment>,
    // TODO: variables
}

impl FeatureFlag {
    /// Builds a feature flag from JSON datafile
    pub fn build(
        datafile: &mut JsonValue,
        rollouts: &mut HashMap<String, Rollout>,
        experiments: &mut HashMap<String, Experiment>,
    ) -> Result<FeatureFlag> {
        let _id = string_field!(datafile, "id")?;
        let key = string_field!(datafile, "key")?;
        let rollout_id = string_field!(datafile, "rolloutId")?;

        // Remove from hashmap to get an owned copy
        let rollout = rollouts
            .remove(&rollout_id)
            .ok_or(DatafileError::InvalidRolloutId(rollout_id))?;

        // Closure to retrieve experiment from HashMap
        let get_experiment = |value: &mut JsonValue| -> Result<Experiment> {
            // TODO: error handling instead of .expect()
            let experiment_id = value.take_string().expect("value should be there");

            // Remove from hashmap to get an owned copy
            let experiment = experiments
                .remove(&experiment_id)
                .ok_or(DatafileError::InvalidExperimentId(experiment_id))?;

            Ok(experiment)
        };

        let experiments = list_field!(datafile, "experimentIds", get_experiment)?;

        let flag = FeatureFlag {
            key,
            rollout,
            experiments,
        };
        Ok(flag)
    }

    pub fn map_entry(self) -> (String, FeatureFlag) {
        (self.key.clone(), self)
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn rollout(&self) -> &Rollout {
        &self.rollout
    }

    pub fn experiments(&self) -> &Vec<Experiment> {
        &self.experiments
    }
}

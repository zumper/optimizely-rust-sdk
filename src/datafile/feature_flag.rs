// External imports
use error_stack::{IntoReport, Report, Result};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

// Imports from crate
use crate::datafile::{DatafileError, Experiment, Rollout};

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
        value: &mut JsonValue,
        rollouts: &mut HashMap<String, Rollout>,
        experiments: &mut HashMap<String, Experiment>,
    ) -> Result<FeatureFlag, DatafileError> {
        let _id = string_field!(value, "id");
        let key = string_field!(value, "key");
        let rollout_id = string_field!(value, "rolloutId");

        // Remove from hashmap to get an owned copy
        let rollout = rollouts
            .remove(&rollout_id)
            .ok_or(Report::new(DatafileError::InvalidRolloutId(rollout_id)))?;

        // Closure to retrieve experiment from HashMap
        let get_experiment = |value: &mut JsonValue| -> Result<Experiment, DatafileError> {
            // Take the experiment ID from the JSON
            let value = value.take();
            let experiment_id = value.as_str().ok_or(DatafileError::MissingExperimentId)?;

            // Remove from hashmap to get an owned copy
            // TODO: look for experiment id in either `groups` of `experiments`
            let experiment = experiments
                .remove(experiment_id)
                .unwrap_or(Experiment::default());

            Ok(experiment)
        };

        // TODO: handle bug where experiment ID can not be found
        let experiments = list_field!(value, "experimentIds", get_experiment);

        let flag = FeatureFlag {
            key: key.to_owned(),
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

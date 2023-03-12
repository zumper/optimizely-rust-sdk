// External imports
use error_stack::{Report, Result};
use std::collections::HashMap;

// Imports from super
use super::{DatafileError, Experiment, Json, Rollout};

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
    pub(crate) fn build(
        json: &mut Json,
        rollouts: &mut HashMap<String, Rollout>,
        experiments: &mut HashMap<String, Experiment>,
    ) -> Result<FeatureFlag, DatafileError> {
        // Get key as String
        let key = json.get("key")?.as_string()?;

        // Get rollout_id as String
        let rollout_id = json.get("rolloutId")?.as_string()?;

        // Remove from hashmap to get an owned copy
        let rollout = rollouts
            .remove(&rollout_id)
            .ok_or_else(|| Report::new(DatafileError::InvalidRolloutId(rollout_id)))?;

        let experiments = json
            .get("experimentIds")?
            .as_array()?
            .map(|json| {
                // Get experiment_id as String
                let experiment_id = json.as_string()?;

                // Remove from HashMap to get an owned copy
                // TODO: look for experiment id in either `groups` of `experiments`
                let experiment = experiments
                    .remove(&experiment_id)
                    .unwrap_or(Experiment::default());

                Ok(experiment)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let flag = FeatureFlag {
            key,
            rollout,
            experiments,
        };
        Ok(flag)
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

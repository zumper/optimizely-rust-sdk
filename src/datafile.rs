//! Everything related to parsing the Optimizely datafile

// External imports
use anyhow::Result;
use std::collections::HashMap;

// Relative imports of sub modules
pub use error::DatafileError;
pub use experiment::Experiment;
pub use feature_flag::FeatureFlag;
pub use rollout::Rollout;
pub use traffic_allocation::TrafficAllocation;
pub use variation::Variation;

mod error;
mod experiment;
mod feature_flag;
mod rollout;
mod traffic_allocation;
mod variation;

#[derive(Debug)]
pub struct Datafile {
    account_id: String,
    revision: u32,
    feature_flags: HashMap<String, FeatureFlag>,
}

impl Datafile {
    pub fn build(datafile: &str) -> Result<Datafile> {
        // Parse datafile as JSON
        let mut datafile = json::parse(datafile)?;

        // Get account id as string
        let account_id = string_field!(datafile, "accountId")?;

        // Get revision as a string and parse to integer
        let revision = string_field!(datafile, "revision")?
            .parse::<u32>()
            .map_err(|_| DatafileError::InvalidRevision)?;

        // Get map of rollouts
        let rollouts: Vec<Rollout> = list_field!(datafile, "rollouts", Rollout::build)?;
        let mut rollouts: HashMap<String, Rollout> = list_to_map!(rollouts, Rollout::map_entry);

        // Get map of experiments
        let experiments: Vec<Experiment> = list_field!(datafile, "experiments", Experiment::build)?;
        let mut experiments: HashMap<String, Experiment> =
            list_to_map!(experiments, Experiment::map_entry);

        // Get map of feature flags
        let build_flag_closure = |value| FeatureFlag::build(value, &mut rollouts, &mut experiments);
        let feature_flags: Vec<FeatureFlag> =
            list_field!(datafile, "featureFlags", build_flag_closure)?;
        let feature_flags: HashMap<String, FeatureFlag> =
            list_to_map!(feature_flags, FeatureFlag::map_entry);

        Ok(Datafile {
            account_id,
            revision,
            feature_flags,
        })
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn revision(&self) -> u32 {
        self.revision
    }

    pub fn feature_flags(&self) -> Vec<&FeatureFlag> {
        self.feature_flags.values().collect()
    }

    pub fn get_flag(&self, flag_key: &str) -> Option<&FeatureFlag> {
        self.feature_flags.get(flag_key)
    }
}

//! Everything related to parsing the Optimizely datafile

// External imports
use std::collections::HashMap;
use std::error::Error;

// Relative imports of sub modules
pub use error::DatafileError;
pub use feature_flag::FeatureFlag;
pub use rollout::Rollout;

mod error;
mod feature_flag;
mod rollout;

#[derive(Debug)]
pub struct Datafile {
    account_id: String,
    revision: u32,
    feature_flags: HashMap<String, FeatureFlag>,
}

impl Datafile {
    pub fn build(datafile: &str) -> Result<Datafile, Box<dyn Error>> {
        // Parse datafile as JSON
        let mut datafile = json::parse(datafile)?;

        // Get account id as string
        let account_id = string_field!(datafile, "accountId")?;

        // Get account id as string
        let revision = string_field!(datafile, "revision")?
            .parse::<u32>()
            .map_err(|_| DatafileError::InvalidRevision)?;

        // Get list of rollouts
        let rollout_closure = |value| Rollout::build(value);
        let rollout_vector = list_field!(datafile, "rollouts", rollout_closure)?;

        // Convert list to hashmap of rollouts, to make it easier to look up
        let mut rollout_map: HashMap<String, Rollout> = rollout_vector
            .into_iter()
            .map(|rollout| (rollout.id.clone(), rollout))
            .collect();

        // Get list of feature flags
        let flag_closure = |value| FeatureFlag::build(value, &mut rollout_map);
        let flag_vector = list_field!(datafile, "featureFlags", flag_closure)?;

        let feature_flags: HashMap<String, FeatureFlag> = flag_vector
            .into_iter()
            .map(|flag| (flag.key.clone(), flag))
            .collect();

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

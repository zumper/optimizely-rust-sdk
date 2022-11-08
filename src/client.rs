//! Everything related to make web requests to Optimizely

// External imports
use std::collections::HashMap;
use std::error::Error;
// Imports from parent
use super::datafile::{DatafileError, FeatureFlag, Rollout};
use super::user_context::UserContext;

#[derive(Debug)]
pub struct Client {
    account_id: String,
    revision: u32,
    feature_flags: Vec<FeatureFlag>,
}

impl Client {
    pub fn build(datafile: &str) -> Result<Client, Box<dyn Error>> {
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
        let feature_flags: Vec<FeatureFlag> = list_field!(datafile, "featureFlags", flag_closure)?;

        Ok(Client {
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

    pub fn feature_flags(&self) -> &Vec<FeatureFlag> {
        &self.feature_flags
    }

    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext {
        UserContext::new(user_id)
    }
}

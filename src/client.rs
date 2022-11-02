// External imports
use std::error::Error;
// Imports from parent
use super::datafile::{DatafileError, FeatureFlag};
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

        // TODO: read out parsed JSON

        // Get account id as string
        let account_id = string_field!(datafile, "accountId")?;

        // Get account id as string
        let revision = string_field!(datafile, "revision")?
            .parse::<u32>()
            .map_err(|_| DatafileError::InvalidRevision)?;

        // Get list of feature flags
        let feature_flags: Vec<FeatureFlag> = datafile["featureFlags"]
            .take()
            .members_mut()
            .map(|value| FeatureFlag::build(value))
            .collect::<Result<Vec<_>, _>>()?;

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

//! Everything related to parsing the Optimizely datafile

// External imports
use error_stack::{IntoReport, Result, ResultExt};
use std::collections::HashMap;

// Relative imports of sub modules
pub use error::DatafileError;
pub use experiment::Experiment;
pub use feature_flag::FeatureFlag;
pub use rollout::Rollout;
pub use traffic_allocation::TrafficAllocation;
pub use variation::Variation;
pub(crate) use json::Json;

mod error;
mod experiment;
mod feature_flag;
mod rollout;
mod traffic_allocation;
mod variation;
mod json;

#[derive(Debug)]
pub struct Datafile {
    account_id: String,
    revision: u32,
    feature_flags: HashMap<String, FeatureFlag>,
}

impl Datafile {
    pub(crate) fn build(json: &mut Json) -> Result<Datafile, DatafileError> {
        // Get account_id as String
        let account_id = json.get("accountId")?
            .as_string()?;

        // Get revision as String, ...
        let revision = json.get("revision")?
            .as_string()?;

        // ... and parse as u32
        let revision = revision.parse()
            .into_report()
            .change_context(DatafileError::InvalidRevision(revision))?;

        // Get HashMap of Rollouts
        let mut rollouts = json.get("rollouts")?
            .as_array()?
            .map(|mut json| Rollout::build(&mut json))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|rollout| (rollout.id().to_owned(), rollout))
            .collect::<HashMap<_, _>>();

        // Get HashMap of Experiments
        let mut experiments = json.get("experiments")?
            .as_array()?
            .map(|mut json| Experiment::build(&mut json))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|experiment| (experiment.id().to_owned(), experiment))
            .collect::<HashMap<_, _>>();

        // // Get Vec of feature flags
        let feature_flags = json.get("featureFlags")?
            .as_array()?
            .map(|mut json| FeatureFlag::build(&mut json, &mut rollouts, &mut experiments))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|flag| (flag.key().to_owned(), flag))
            .collect::<HashMap<_, _>>();

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

    pub fn get_flag(&self, flag_key: &str) -> Option<&FeatureFlag> {
        self.feature_flags.get(flag_key)
    }
}

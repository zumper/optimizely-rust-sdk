//! Parsing the Optimizely datafile

// External imports
use error_stack::{IntoReport, Result, ResultExt};

// Relative imports of sub modules
use environment::Environment;
pub use error::DatafileError;
use event::Event;
pub(crate) use experiment::Experiment;
pub(crate) use feature_flag::FeatureFlag;
use rollout::Rollout;
use traffic_allocation::TrafficAllocation;
pub(crate) use variation::Variation;

mod environment;
mod error;
mod event;
mod experiment;
mod feature_flag;
mod rollout;
mod traffic_allocation;
mod variation;

#[derive(Debug)]
pub(crate) struct Datafile(Environment);

impl Datafile {
    pub(crate) fn build(content: &str) -> Result<Datafile, DatafileError> {
        // Parse the JSON content via serde into Rust structs
        let environment: Environment = serde_json::from_str(content)
            .into_report()
            .change_context(DatafileError::InvalidJson)?;

        Ok(Datafile(environment))
    }

    pub fn account_id(&self) -> &str {
        self.0.account_id()
    }

    pub fn revision(&self) -> u32 {
        self.0.revision()
    }

    pub fn flag(&self, flag_key: &str) -> Option<&FeatureFlag> {
        self.0.feature_flags().get(flag_key)
    }

    pub fn experiment(&self, experiment_id: &str) -> Option<&Experiment> {
        self.0.experiments().get(experiment_id)
    }

    pub fn rollout(&self, rollout_id: &str) -> Option<&Rollout> {
        self.0.rollouts().get(rollout_id)
    }

    pub fn event(&self, event_key: &str) -> Option<&Event> {
        self.0.events().get(event_key)
    }
}

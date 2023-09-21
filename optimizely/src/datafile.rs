//! Parsing the Optimizely datafile

// External imports
use error_stack::{IntoReport, Result, ResultExt};
use std::collections::HashMap;

// Relative imports of sub modules
pub(crate) use context::Context;
pub use error::DatafileError;
#[cfg(feature = "online")]
pub(crate) use event::Event;
pub(crate) use experiment::Experiment;
pub(crate) use feature_flag::FeatureFlag;
pub(crate) use rollout::Rollout;
pub(crate) use traffic_allocation::TrafficAllocation;
pub(crate) use variation::Variation;

mod context;
mod error;
#[cfg(feature = "online")]
mod event;
mod experiment;
mod feature_flag;
mod rollout;
mod traffic_allocation;
mod variation;

#[derive(Debug)]
pub(crate) struct Datafile {
    account_id: String,
    revision: u32,
    feature_flags: HashMap<String, FeatureFlag>,
    #[cfg(feature = "online")]
    events: HashMap<String, Event>,
}

impl Datafile {
    pub(crate) fn build(content: &str) -> Result<Datafile, DatafileError> {
        // Parse content into a context
        let mut context = Context::build(content).change_context(DatafileError::InvalidJson)?;

        // Get account_id as String
        let account_id = context.get("accountId")?.as_string()?;

        // Get revision as String, ...
        let revision = context.get("revision")?.as_string()?;

        // ... and parse as u32
        let revision = revision
            .parse()
            .into_report()
            .change_context(DatafileError::InvalidRevision(revision))?;

        #[cfg(feature = "online")]
        let events = context
            .get("events")?
            .as_array()?
            .map(|mut context| Event::build(&mut context))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|event| (event.key().to_owned(), event))
            .collect::<HashMap<_, _>>();

        // Get HashMap of Rollouts
        let mut rollouts = context
            .get("rollouts")?
            .as_array()?
            .map(|mut context| Rollout::build(&mut context))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|rollout| (rollout.id().to_owned(), rollout))
            .collect::<HashMap<_, _>>();

        // Get HashMap of Experiments
        let mut experiments = context
            .get("experiments")?
            .as_array()?
            .map(|mut context| Experiment::build(&mut context))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|experiment| (experiment.id().to_owned(), experiment))
            .collect::<HashMap<_, _>>();

        // // Get Vec of feature flags
        let feature_flags = context
            .get("featureFlags")?
            .as_array()?
            .map(|mut context| FeatureFlag::build(&mut context, &mut rollouts, &mut experiments))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|flag| (flag.key().to_owned(), flag))
            .collect::<HashMap<_, _>>();

        Ok(Datafile {
            account_id,
            revision,
            feature_flags,
            #[cfg(feature = "online")]
            events,
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

    #[cfg(feature = "online")]
    pub fn get_event(&self, event_key: &str) -> Option<&Event> {
        self.events.get(event_key)
    }
}

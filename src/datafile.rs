//! Everything related to parsing the Optimizely datafile

// External imports
use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

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
    pub fn build_from_sdk_key(sdk_key: &str) -> Result<Datafile> {
        // Construct URL
        let url = format!("https://cdn.optimizely.com/datafiles/{}.json", sdk_key);

        // Make GET request
        // TODO: implement polling mechanism
        let response = ureq::get(&url)
            .call()
            .or_else(|_| Err(DatafileError::FailedRequest))?;

        // Get response body
        let content = response
            .into_string()
            .or_else(|_| Err(DatafileError::FailedResponse))?;

        // Use response to build Client
        Datafile::build_from_string(content)
    }

    pub fn build_from_file(file_path: &str) -> Result<Datafile> {
        // Read content from local path
        let mut content = String::new();

        // Open file
        let mut file = File::open(file_path).or_else(|_| Err(DatafileError::FailedFileOpen))?;

        // Read file content into String
        file.read_to_string(&mut content)
            .or_else(|_| Err(DatafileError::FailedFileRead))?;

        // Use file content to build Client
        Datafile::build_from_string(content)
    }

    pub fn build_from_string(content: String) -> Result<Datafile> {
        // Parse content as JSON
        let mut value = json::parse(&content)?;

        // Get account id as string
        let account_id = string_field!(value, "accountId")?;

        // Get revision as a string and parse to integer
        let revision = string_field!(value, "revision")?
            .parse::<u32>()
            .map_err(|_| DatafileError::InvalidRevision)?;

        // Get map of rollouts
        let rollouts: Vec<Rollout> = list_field!(value, "rollouts", Rollout::build)?;
        let mut rollouts: HashMap<String, Rollout> = list_to_map!(rollouts, Rollout::map_entry);

        // Get map of experiments
        let experiments: Vec<Experiment> = list_field!(value, "experiments", Experiment::build)?;
        let mut experiments: HashMap<String, Experiment> = list_to_map!(experiments, Experiment::map_entry);

        // Get map of feature flags
        let build_flag_closure = |value| FeatureFlag::build(value, &mut rollouts, &mut experiments);
        let feature_flags: Vec<FeatureFlag> = list_field!(value, "featureFlags", build_flag_closure)?;
        let feature_flags: HashMap<String, FeatureFlag> = list_to_map!(feature_flags, FeatureFlag::map_entry);

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

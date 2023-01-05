// External imports
use anyhow::Result;
use fasthash::murmur3::hash32_with_seed as murmur3_hash;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::SystemTime;

// Imports from parent
use super::datafile::{Datafile, Experiment, FeatureFlag, Variation};
use super::decision::{DecideOption, Decision};

// Custom type alias
pub type UserAttributes = HashMap<String, String>;

// Constant used for the hashing algorithm
const HASH_SEED: u32 = 1;
// Ranges are specified between 0 and 10_000
const MAX_OF_RANGE: f64 = 10_000 as f64;

#[derive(Debug)]
pub struct UserContext {
    datafile: Rc<Datafile>,
    user_id: String,
    attributes: UserAttributes,
}

impl UserContext {
    pub fn new(datafile: &Rc<Datafile>, user_id: &str) -> UserContext {
        // Create a clone of the reference, thus increasing the count
        let datafile = Rc::clone(&datafile);

        // Create owned copy of user_id
        let user_id = user_id.to_owned();

        // Create an empty set of user attributes
        let attributes = UserAttributes::new();

        UserContext {
            datafile,
            user_id,
            attributes,
        }
    }

    // TODO: add pub fn new_with_attributes

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        // Create owned copies of the key and value
        let key = key.to_owned();
        let value = value.to_owned();

        // Add the attribute
        self.attributes.insert(key, value);
    }

    pub fn get_attributes(&self) -> &UserAttributes {
        // Return borrowed reference to attributes
        &self.attributes
    }

    pub fn decide<'a, 'b>(&'a self, flag_key: &'b str, options: &Vec<DecideOption>) -> Decision<'b> {
        // Retrieve Flag object
        let flag = match self.datafile.get_flag(flag_key) {
            Some(flag) => flag,
            None => {
                // When flag key cannot be found, return the off variation
                // CONSIDERATION: Could have used Result<Decision, E> but this is how other Optimizely SDKs work
                return Decision::off(flag_key);
            }
        };

        // Only send decision events if the DisableDecisionEvent option is not included
        let send_decision = !options
            .iter()
            .any(|option| *option == DecideOption::DisableDecisionEvent);

        // Get the selected variation for the given flag
        match self.get_variation_for_flag(flag, send_decision) {
            Some(variation) => {
                // Unpack the variation and create Decision struct
                Decision::new(flag_key, variation.is_feature_enabled(), variation.key().to_owned())
            }
            None => {
                // No experiment or rollout found, or user does not qualify for any
                Decision::off(flag_key)
            }
        }
    }

    fn get_variation_for_flag(&self, flag: &FeatureFlag, send_decision: bool) -> Option<Rc<Variation>> {
        // TODO: don't send decision if DecideOption.DisableDecisionEvent is set

        // Find first Experiment for which this user qualifies
        let result = flag
            .experiments()
            .iter()
            .find_map(|experiment| self.get_variation_for_experiment(experiment, send_decision));

        match result {
            Some(_) => {
                // A matching A/B test was found, send out any decisions
                result
            }
            None => {
                // No direct experiment found, let's look at the Rollout

                // Find the first experiment within the Rollout for which this user qualifies
                flag.rollout()
                    .experiments()
                    .iter()
                    .find_map(|experiment| self.get_variation_for_experiment(experiment, false))
            }
        }
    }

    fn get_variation_for_experiment(&self, experiment: &Experiment, send_decision: bool) -> Option<Rc<Variation>> {
        // Use references for the ids
        let user_id = &self.user_id;
        let experiment_id = &experiment.id();

        // Concatenate user id and experiment id
        let bucketing_key = format!("{user_id}{experiment_id}");

        // To hash the bucket key it needs to be converted to an array of `u8` bytes
        // Use Murmur3 (32-bit) with seed
        let hash_value = murmur3_hash(bucketing_key.as_bytes(), HASH_SEED);

        // Bring the hash into a range of 0 to 10_000
        let bucket_value = ((hash_value as f64) / (u32::MAX as f64) * MAX_OF_RANGE) as u32;

        // Get the variation according to the traffic allocation
        let result = experiment
            .traffic_allocation()
            .get_variation_for_bucket(bucket_value);

        match result {
            Some(variation) => {
                if send_decision {
                    // Send out a decision event as a side effect
                    // Ignore result of the send_decision function
                    let _ = self.send_decision(experiment, Rc::clone(&variation));
                }
                Some(variation)
            }
            None => None,
        }
    }

    fn send_decision(&self, experiment: &Experiment, variation: Rc<Variation>) -> Result<()> {
        // Get timestamp as milliseconds since the epoch
        // NOTE: Convert to u64 as json::object does not support u128
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis() as u64;

        // Decision object
        let decision = json::object! {
            "campaign_id": experiment.campaign_id().to_owned(),
            "experiment_id": experiment.id().to_owned(),
            "variation_id": variation.id().to_owned(),
            "is_campaign_holdback": false,
        };

        // Event object
        let event = json::object! {
            "entity_id": experiment.campaign_id().to_owned(),
            "type": "campaign_activated",
            "timestamp": timestamp,
            "uuid": 1,
        };

        // Snapshot object
        let snapshot = json::object! {
            "decisions": [decision],
            "events": [event],
        };

        // Visitor object
        let visitor = json::object! {
            "visitor_id": self.user_id.to_owned(),
            "snapshots": [snapshot],
        };

        // TODO: queue these decisions and send in batches

        // POST request payload
        let payload = json::object! {
            "account_id": self.datafile.account_id().to_owned(),
            "visitors": [visitor],
            "enrich_decisions": true,
            "anonymize_ip": true,
            "client_name": "rust-sdk",
            "client_version": "0.0.1",
        };

        // Make POST request
        let response = ureq::post("https://logx.optimizely.com/v1/events")
            .set("content-type", "application/json")
            .send_string(&payload.dump())?;

        // Ignore response
        drop(response);

        Ok(())
    }
}

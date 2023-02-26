// External imports
use fasthash::murmur3::hash32_with_seed as murmur3_hash;
use std::collections::HashMap;
use std::rc::Rc;

// Imports from parent
use super::datafile::{Experiment, FeatureFlag, Variation};
use super::decision::{DecideOption, Decision};
use super::Client;

// Custom type alias
pub type UserAttributes = HashMap<String, String>;

// Constant used for the hashing algorithm
const HASH_SEED: u32 = 1;

// Ranges are specified between 0 and 10_000
const MAX_OF_RANGE: f64 = 10_000 as f64;


#[derive(Debug)]
pub struct UserContext<'a> {
    client: &'a Client,
    user_id: &'a str,
    attributes: UserAttributes,
}

impl UserContext<'_> {
    pub fn new<'a>(client: &'a Client, user_id: &'a str) -> UserContext<'a> {
        // Create an empty set of user attributes
        let attributes = UserAttributes::new();

        UserContext {
            client,
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

    /// Getter for `user_id` field
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn attributes(&self) -> &UserAttributes {
        // Return borrowed reference to attributes
        &self.attributes
    }


    pub fn decide<'a, 'b>(&'a self, flag_key: &'b str, options: &Vec<DecideOption>) -> Decision<'b> {
        // Retrieve Flag object
        let flag = match self.client.datafile.get_flag(flag_key) {
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
                    self.client.event_dispatcher.send_decision(self, experiment, Rc::clone(&variation));
                }
                Some(variation)
            }
            None => None,
        }
    }
}

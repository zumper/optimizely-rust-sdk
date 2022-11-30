// External imports
use fasthash::murmur3::hash32_with_seed as murmur3_hash;
use std::collections::HashMap;
use std::rc::Rc;

// Imports from parent
use super::datafile::{Datafile, Experiment, Variation};
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

    pub fn decide<'a, 'b>(
        &'a self,
        flag_key: &'b str,
        _options: &Vec<DecideOption>,
    ) -> Decision<'b> {
        // Retrieve Flag object
        let flag = match self.datafile.get_flag(flag_key) {
            Some(flag) => flag,
            None => {
                // When flag key cannot be found, return the off variation
                // CONSIDERATION: Could have used Result<Decision, E> but this is how other Optimizely SDKs work
                return Decision::off(flag_key);
            }
        };

        // Find first Experiment for which this user qualifies
        let result = flag
            .experiments
            .iter()
            .find_map(|experiment| self.decide_for_experiment(experiment));

        match result {
            Some(variation) => {
                // Unpack the variation and create Decision struct
                Decision::new(
                    flag_key,
                    variation.is_feature_enabled,
                    variation.key.to_owned(),
                )
            }
            None => {
                // No direct experiment found, let's look at the Rollout

                // Find the first experiment within the Rollout for which this user qualifies
                let result = flag
                    .rollout
                    .experiments
                    .iter()
                    .find_map(|experiment| self.decide_for_experiment(experiment));

                match result {
                    Some(variation) => {
                        // Unpack the variation and create Decision struct
                        Decision::new(
                            flag_key,
                            variation.is_feature_enabled,
                            variation.key.to_owned(),
                        )
                    }
                    None => {
                        // No experiment or rollout found, or user does not qualify for any
                        Decision::off(flag_key)
                    }
                }
            }
        }
    }

    fn decide_for_experiment<'a>(&'a self, experiment: &'a Experiment) -> Option<Rc<Variation>> {
        // Use references for the ids
        let user_id = &self.user_id;
        let experiment_id = &experiment.id;

        // Concatenate user id and experiment id
        let bucketing_key = format!("{user_id}{experiment_id}");

        // To hash the bucket key it needs to be converted to an array of `u8` bytes
        // Use Murmur3 (32-bit) with seed
        let hash_value = murmur3_hash(bucketing_key.as_bytes(), HASH_SEED);

        // Bring the hash into a range of 0 to 10_000
        let bucket_value = ((hash_value as f64) / (u32::MAX as f64) * MAX_OF_RANGE) as u32;

        // Get the variation according to the traffic allocation
        experiment
            .traffic_allocation
            .get_variation_for_bucket(bucket_value)
    }
}

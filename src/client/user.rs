// External imports
use fasthash::murmur3::hash32_with_seed as murmur3_hash;
use std::collections::HashMap;

// Imports from crate
use crate::datafile::{Experiment, FeatureFlag, Variation};
use crate::decision::{DecideOptions, Decision};

#[cfg(feature = "online")]
use crate::event::Event;

// Imports from super
use super::Client;

/// Custom type alias for user attributes
pub type UserAttributes = HashMap<String, String>;

/// Constant used for the hashing algorithm
const HASH_SEED: u32 = 1;

/// Ranges are specified between 0 and 10_000
const MAX_OF_RANGE: f64 = 10_000_f64;

/// User specific context
///
/// Foo
pub struct UserContext<'a> {
    client: &'a Client,
    user_id: &'a str,
    attributes: UserAttributes,
}

impl UserContext<'_> {
    pub(crate) fn new<'a>(client: &'a Client, user_id: &'a str) -> UserContext<'a> {
        // Create an empty set of user attributes
        let attributes = UserAttributes::new();

        UserContext {
            client,
            user_id,
            attributes,
        }
    }

    // TODO: add pub fn new_with_attributes

    /// Add a new attribute to a user context
    pub fn set_attribute(&mut self, key: &str, value: &str) {
        // Create owned copies of the key and value
        let key = key.to_owned();
        let value = value.to_owned();

        // Add the attribute
        self.attributes.insert(key, value);
    }

    /// Get the parent client of a user context
    #[allow(dead_code)]
    pub(crate) fn client(&self) -> &Client {
        self.client
    }

    /// Get the id of a user
    pub fn user_id(&self) -> &str {
        self.user_id
    }

    /// Get all attributes of a user
    pub fn attributes(&self) -> &UserAttributes {
        // Return borrowed reference to attributes
        &self.attributes
    }

    /// Decide which variation to show to a user
    ///
    /// ```
    /// # use optimizely::ClientBuilder;
    /// # let file_path = "examples/datafiles/sandbox.json";
    /// # let optimizely_client = ClientBuilder::new()
    /// #     .with_local_datafile(file_path).unwrap()
    /// #     .build().unwrap();
    /// #
    /// let user_context = optimizely_client.create_user_context("123abc789xyz");
    ///
    /// let decision = user_context.decide("buy_button");
    /// ```
    pub fn decide<'b>(&self, flag_key: &'b str) -> Decision<'b> {
        let options = DecideOptions::default();
        self.decide_with_options(flag_key, &options)
    }

    /// Decide which variation to show to a user
    pub fn decide_with_options<'b>(&self, flag_key: &'b str, options: &DecideOptions) -> Decision<'b> {
        // Retrieve Flag object
        let flag = match self.client.datafile().get_flag(flag_key) {
            Some(flag) => flag,
            None => {
                // When flag key cannot be found, return the off variation
                // CONSIDERATION: Could have used Result<Decision, E> but this is how other Optimizely SDKs work
                return Decision::off(flag_key);
            }
        };

        // Only send decision events if the disable_decision_event option is false
        let send_decision = !options.disable_decision_event;

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

    fn get_variation_for_flag<'a>(&'a self, flag: &'a FeatureFlag, send_decision: bool) -> Option<&Variation> {
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

    fn get_variation_for_experiment<'a>(
        &'a self, experiment: &'a Experiment, send_decision: bool,
    ) -> Option<&Variation> {
        // Use references for the ids
        let user_id = self.user_id();
        let experiment_id = experiment.id();

        // Concatenate user id and experiment id
        let bucketing_key = format!("{user_id}{experiment_id}");

        // To hash the bucket key it needs to be converted to an array of `u8` bytes
        // Use Murmur3 (32-bit) with seed
        let hash_value = murmur3_hash(bucketing_key.as_bytes(), HASH_SEED);

        // Bring the hash into a range of 0 to 10_000
        let bucket_value = ((hash_value as f64) / (u32::MAX as f64) * MAX_OF_RANGE) as u64;

        // Get the variation according to the traffic allocation
        let result = experiment
            .traffic_allocation()
            .get_variation_for_bucket(bucket_value);

        match result {
            Some(variation) => {
                if send_decision {
                    #[cfg(feature = "online")]
                    {
                        // Send out a decision event as a side effect
                        let account_id = self.client().account_id();
                        let campaign_id = experiment.campaign_id();
                        let variation_id = variation.id();
                        let event = Event::decision(account_id, user_id, campaign_id, experiment_id, variation_id);

                        // Ignore result of the send_decision function
                        self.client.event_dispatcher().send_event(event);
                    }
                }
                Some(variation)
            }
            None => None,
        }
    }
}

/// Macro to create UserAttributes
#[macro_export]
macro_rules! user_attributes {
    { $( $key: expr => $value: expr),* $(,)?} => {
        {
            let mut attribute = UserAttributes::new();

            $(
                attribute.insert($key.to_owned(), $value.to_owned());
            )*

            attribute
        }
    };
}

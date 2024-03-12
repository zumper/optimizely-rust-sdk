// External imports
use murmur3::murmur3_32 as murmur3_hash;
use serde::Serialize;
use serde_json::value::Number;
use std::collections::HashMap;
use std::io::Cursor;

// Imports from crate
use crate::datafile::{Experiment, FeatureFlag, Variation};
use crate::decision::{DecideOptions, Decision};

#[cfg(feature = "online")]
use crate::event_api;

// Imports from super
use super::Client;

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    Null,
    String(String),
    Number(Number),
    Bool(bool),
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        AttributeValue::Bool(value)
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        AttributeValue::String(value.into())
    }
}

impl From<i32> for AttributeValue {
    fn from(value: i32) -> Self {
        AttributeValue::Number(value.into())
    }
}

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
        if let Some(number) = Number::from_f64(value) {
            return AttributeValue::Number(number);
        }
        log::warn!("NaN and +/- Infinity are not supported in AttributeValue::Number");
        AttributeValue::Null
    }
}

impl Serialize for AttributeValue {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AttributeValue::Null => serializer.serialize_none(),
            AttributeValue::Bool(value) => value.serialize(serializer),
            AttributeValue::String(value) => value.serialize(serializer),
            AttributeValue::Number(value) => value.serialize(serializer),
        }
    }
}

impl AttributeValue {
    /// If the `Value` is a Boolean, returns the associated bool. Returns None
    /// otherwise.
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            AttributeValue::Bool(b) => Some(b),
            _ => None,
        }
    }
    /// If the `Value` is a String, returns the associated str. Returns None
    /// otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttributeValue::String(s) => Some(s),
            _ => None,
        }
    }
    /// If the `Value` is a Number, returns the associated [`Number`]. Returns
    /// None otherwise.
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            AttributeValue::Number(number) => Some(number),
            _ => None,
        }
    }
    /// Returns true if the `Value` is Null
    pub fn is_null(&self) -> bool {
        matches!(self, AttributeValue::Null)
    }
}

/// Custom type alias for user attributes
pub type UserAttributes = HashMap<String, AttributeValue>;

/// Constant used for the hashing algorithm
const HASH_SEED: u32 = 1;

/// Ranges are specified between 0 and 10_000
const MAX_OF_RANGE: f64 = 10_000_f64;

/// User specific context
///
/// ```
/// use optimizely::{Client, decision::DecideOptions};
///
/// // Initialize Optimizely client using local datafile
/// let file_path = "../datafiles/sandbox.json";
/// let optimizely_client = Client::from_local_datafile(file_path)?
///     .initialize();
///
/// // Do not send any decision events
/// let decide_options = DecideOptions {
///     disable_decision_event: true,
///     ..DecideOptions::default()
/// };
///
/// // Create a user context
/// let attributes = optimizely::user_attributes! {
///     "is_employee" => "true",
///     "app_version" => "1.3.2",
/// };
/// let user_context = optimizely_client.create_user_context("123abc789xyz");
///
/// // Decide a feature flag for this user
/// let decision = user_context.decide_with_options("buy_button", &decide_options);
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct UserContext<'a> {
    client: &'a Client,
    user_id: &'a str,
    attributes: UserAttributes,
}

impl UserContext<'_> {
    // Only allow UserContext to be constructed from a Client
    pub(crate) fn new<'a>(client: &'a Client, user_id: &'a str, attributes: UserAttributes) -> UserContext<'a> {
        UserContext {
            client,
            user_id,
            attributes,
        }
    }

    /// Add a new attribute to a user context
    pub fn set_attribute<K: Into<String>, V: Into<AttributeValue>>(&mut self, key: K, value: V) {
        // Create owned copies of the key and value
        let key = key.into();

        // Add the attribute
        self.attributes.insert(key, value.into());
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

    #[cfg(feature = "online")]
    /// Track a conversion event for this user
    pub fn track_event(&self, event_key: &str) {
        match self.client.datafile().event(event_key) {
            Some(event) => {
                log::debug!("Logging conversion event");

                // Send out a decision event as a side effect
                let user_id = self.user_id();
                let account_id = self.client.datafile().account_id();
                let event_id = event.id();

                // Create event_api::Event to send to dispatcher
                let conversion_event = event_api::Event::conversion(account_id, user_id, event_id, event_key);

                // Ignore result of the send_decision function
                self.client.event_dispatcher().send_event(conversion_event);
            }
            None => {
                log::warn!("Event key does not exist in datafile");
            }
        }
    }

    /// Decide which variation to show to a user
    pub fn decide<'b>(&self, flag_key: &'b str) -> Decision<'b> {
        let options = DecideOptions::default();
        self.decide_with_options(flag_key, &options)
    }

    /// Decide which variation to show to a user
    pub fn decide_with_options<'b>(&self, flag_key: &'b str, options: &DecideOptions) -> Decision<'b> {
        // Retrieve Flag object
        let flag = match self.client.datafile().flag(flag_key) {
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
        match self.decide_variation_for_flag(flag, send_decision) {
            Some(variation) => {
                // Unpack the variation and create Decision struct
                Decision::new(flag_key, variation.is_feature_enabled(), variation.key())
            }
            None => {
                // No experiment or rollout found, or user does not qualify for any
                Decision::off(flag_key)
            }
        }
    }

    fn decide_variation_for_flag<'a>(&'a self, flag: &'a FeatureFlag, send_decision: bool) -> Option<&Variation> {
        // Find first Experiment for which this user qualifies
        let result = flag.experiments_ids().iter().find_map(|experiment_id| {
            let experiment = self.client.datafile().experiment(experiment_id);

            match experiment {
                Some(experiment) => self.decide_variation_for_experiment(experiment, send_decision),
                None => None,
            }
        });

        match result {
            Some(_) => {
                // A matching A/B test was found, send out any decisions
                result
            }
            None => {
                // No direct experiment found, let's look at the Rollout
                let rollout = self.client.datafile().rollout(flag.rollout_id()).unwrap(); // TODO: remove unwrap

                // Find the first experiment within the Rollout for which this user qualifies
                rollout
                    .experiments()
                    .iter()
                    .find_map(|experiment| self.decide_variation_for_experiment(experiment, false))
            }
        }
    }

    pub fn decide_variation_for_experiment<'a>(
        &'a self, experiment: &'a Experiment, send_decision: bool,
    ) -> Option<&Variation> {
        // Use references for the ids
        let user_id = self.user_id();
        let experiment_id = experiment.id();

        // Concatenate user id and experiment id
        let bucketing_key = format!("{user_id}{experiment_id}");

        // To hash the bucket key it needs to be converted to an array of `u8` bytes
        // Use Murmur3 (32-bit) with seed
        let hash_value = match murmur3_hash(&mut Cursor::new(&bucketing_key), HASH_SEED) {
            Ok(value) => value,
            Err(_e) => {
                log::warn!("Unable to create hash for bucketing_key={}", &bucketing_key);
                return None;
            }
        };

        // Bring the hash into a range of 0 to 10_000
        let bucket_value = ((hash_value as f64) / (u32::MAX as f64) * MAX_OF_RANGE) as u64;

        // Get the variation according to the traffic allocation
        let result = experiment.traffic_allocation().variation(bucket_value);

        match result {
            Some(variation_id) => {
                if send_decision {
                    #[cfg(feature = "online")]
                    {
                        // Send out a decision event as a side effect
                        let account_id = self.client.datafile().account_id();
                        let campaign_id = experiment.campaign_id();

                        // Create event_api::Event to send to dispatcher
                        let decision_event =
                            event_api::Event::decision(account_id, user_id, campaign_id, experiment_id, variation_id);

                        // Ignore result of the send_decision function
                        self.client.event_dispatcher().send_event(decision_event);
                    }
                }

                // Find the variation belonging to this variation ID
                experiment.variation(variation_id)
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
            let mut attribute = optimizely::client::UserAttributes::new();

            $(
                attribute.insert($key.into(), $value.into());
            )*

            attribute
        }
    };
}

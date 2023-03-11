// External imports
use error_stack::{IntoReport, Result};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::rc::Rc;

// Imports from crate
use crate::datafile::DatafileError;

/// A single variation like "off", "on" or other user-created variations.
///
/// A variation has the properties `id`, `key`, and `is_feature_enabled`.
/// The `id` is a unique identifier.
/// The `key` is a human-readable value.
/// The value of `is_feature_enabled` is `false` for the "off" variation.
/// All other variations will have `is_feature_enabled` is `true`.
#[derive(Debug)]
pub struct Variation {
    id: String,
    key: String,
    is_feature_enabled: bool,
}

impl Variation {
    /// Create a new variation.
    ///
    /// # Examples
    /// ```rust
    /// use optimizely::datafile::Variation;
    ///
    /// let variation = Variation::new("58054".into(), "on".into(), true);
    ///
    /// assert_eq!(variation.key(), "on");
    /// assert_eq!(variation.is_feature_enabled(), true);
    /// ```
    pub fn new(id: String, key: String, is_feature_enabled: bool) -> Variation {
        Variation {
            id,
            key,
            is_feature_enabled,
        }
    }

    /// Create a new variation from a JSON value.
    ///
    /// # Examples
    /// ```rust
    /// use optimizely::datafile::Variation;
    ///
    /// let mut value = serde_json::json!({
    ///     "id": "58054",
    ///     "key": "on",
    ///     "featureEnabled": true,
    /// });
    ///
    /// let variation = Variation::build(&mut value).unwrap();
    ///
    /// assert_eq!(variation.key(), "on");
    /// assert_eq!(variation.is_feature_enabled(), true);
    /// ```
    pub fn build(value: &mut JsonValue) -> Result<Variation, DatafileError> {
        let id = string_field!(value, "id");
        let key = string_field!(value, "key");

        // TODO: fix bug below again
        // BUG: Found an example datafile where this field is missing, therefore default to `false`
        let is_feature_enabled = bool_field!(value, "featureEnabled");

        Ok(Variation::new(id, key, is_feature_enabled))
    }

    /// Converts a list of variations to a HashMap
    pub(crate) fn list_to_map(variations: Vec<Variation>) -> HashMap<String, Rc<Variation>> {
        variations
            .into_iter()
            .map(|variation| (variation.id.to_owned(), Rc::new(variation)))
            .collect()
    }

    /// Getter for `id` field
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Getter for `key` field
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Getter for `is_feature_enabled` field
    pub fn is_feature_enabled(&self) -> bool {
        self.is_feature_enabled
    }
}

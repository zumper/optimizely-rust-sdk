//! Result of a feature flag

// Relative imports of sub modules
pub use decide_options::DecideOptions;
mod decide_options;

/// Decision for a specfic user and feature flag
#[derive(Debug)]
pub struct Decision<'a> {
    flag_key: &'a str,
    enabled: bool,
    variation_key: String,
}

impl Decision<'_> {
    pub(crate) fn new<T: Into<String>>(flag_key: &str, enabled: bool, variation_key: T) -> Decision {
        Decision {
            flag_key,
            enabled,
            variation_key: variation_key.into(),
        }
    }

    pub(crate) fn off(flag_key: &str) -> Decision {
        Decision::new(flag_key, false, "off")
    }

    /// Get the flag key for which this decision was made
    pub fn flag_key(&self) -> &str {
        self.flag_key
    }

    /// Get whether the flag should be enabled or disable
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Get the variation key that was decided
    pub fn variation_key(&self) -> &str {
        &self.variation_key
    }
}

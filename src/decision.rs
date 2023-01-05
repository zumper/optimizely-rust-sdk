// Relative imports of sub modules
pub use decide_option::DecideOption;
mod decide_option;

// Inspiration: https://docs.developers.optimizely.com/experimentation/v4.0.0-full-stack/docs/optimizelydecision-python

#[derive(Debug)]
pub struct Decision<'a> {
    flag_key: &'a str,
    enabled: bool,
    // rule_key
    variation_key: String,
    // variables
    // user_context
    // reasons
}

impl Decision<'_> {
    pub fn new<'a>(flag_key: &'a str, enabled: bool, variation_key: String) -> Decision {
        Decision {
            flag_key,
            enabled,
            variation_key,
        }
    }

    pub fn off<'a>(flag_key: &'a str) -> Decision {
        Decision::new(flag_key, false, String::from("off"))
    }

    pub fn flag_key(&self) -> &str {
        &self.flag_key
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn variation_key(&self) -> &str {
        &self.variation_key
    }
}

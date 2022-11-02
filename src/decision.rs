// Relative imports of sub modules
pub use decide_option::DecideOption;
mod decide_option;

// Inspiration: https://docs.developers.optimizely.com/experimentation/v4.0.0-full-stack/docs/optimizelydecision-python

#[derive(Debug)]
pub struct Decision<'a> {
    pub flag_key: &'a str,
    // rule_key
    pub variation_key: &'a str,
    pub enabled: bool,
    // variables
    // user_context
    // reasons
}

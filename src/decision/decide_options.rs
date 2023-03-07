use std::default::Default;

#[derive(Debug)]
pub struct DecideOptions {
    // Prevents the visitor from firing an impression while still being served the variation,
    // which disables displaying results of the Decide method on the Optimizely application's Results page.
    // This setting can be why the Decision Event Dispatched enum is false in the returned OptimizelyDecision object or the DECIDE notification listener payload.
    pub disable_decision_event: bool,

    // Return decisions for enabled flags only.
    // This is a valid option only for methods that decide multiple flags, like the Decide All method.
    // This option is ignored if it is invalid.
    // When this option is not set, the SDK returns all decisions regardless of whether the flag is enabled or not.
    pub enabled_flags_only: bool,

    // When set, the SDK bypasses UPS (both lookup and save) for the decision.
    // When this option is not set, UPS overrides audience targeting, traffic allocation, and experiment mutual exclusion groups.
    pub ignore_user_profile_service: bool,

    // Return log messages in the Reasons field of OptimizelyDecision object. Note that unlike info or debug messages, critical error messages are always returned, regardless of this setting.
    pub include_reasons: bool,

    // Exclude flag variable values from the decision result. Use this option to minimize the returned decision by skipping large JSON variables.
    pub exclude_variables: bool,
}

impl Default for DecideOptions {
    fn default() -> Self {
        // By default, all options are false
        DecideOptions {
            disable_decision_event: false,
            enabled_flags_only: false,
            ignore_user_profile_service: false,
            include_reasons: false,
            exclude_variables: false,
        }
    }
}

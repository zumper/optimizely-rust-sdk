#[derive(Debug, PartialEq)]
pub enum DecideOption {
    // Prevents the visitor from firing an impression while still being served the variation,
    // which disables displaying results of the Decide method on the Optimizely application's Results page.
    // This setting can be why the Decision Event Dispatched enum is false in the returned OptimizelyDecision object or the DECIDE notification listener payload.
    DisableDecisionEvent,

    // Return decisions for enabled flags only.
    // This is a valid option only for methods that decide multiple flags, like the Decide All method.
    // This option is ignored if it is invalid.
    // When this option is not set, the SDK returns all decisions regardless of whether the flag is enabled or not.
    EnabledFlagsOnly,

    // When set, the SDK bypasses UPS (both lookup and save) for the decision.
    // When this option is not set, UPS overrides audience targeting, traffic allocation, and experiment mutual exclusion groups.
    IgnoreUserProfileService,

    // Return log messages in the Reasons field of OptimizelyDecision object. Note that unlike info or debug messages, critical error messages are always returned, regardless of this setting.
    IncludeReasons,

    // Exclude flag variable values from the decision result. Use this option to minimize the returned decision by skipping large JSON variables.
    ExcludeVariables,
}

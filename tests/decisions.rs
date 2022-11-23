// Relative imports of sub modules
use common::get_decision;
mod common;

#[test]
fn qa_rollout_flag() {
    let flag_key = "qa_rollout";

    // The flag should be off for `user1 `
    let decision = get_decision("user1", flag_key);
    assert!(!decision.enabled());
    assert_eq!(decision.variation_key(), "off");

    // The flag should be on for `user3 `
    let decision = get_decision("user3", flag_key);
    assert!(decision.enabled());
    assert_eq!(decision.variation_key(), "on");
}

#[test]
fn invalid_flag() {
    let flag_key = "this_flag_does_not_exist";

    // An invalid flag should always be disabled
    let decision = get_decision("_", flag_key);
    assert!(!decision.enabled());
    assert_eq!(decision.variation_key(), "off");
}

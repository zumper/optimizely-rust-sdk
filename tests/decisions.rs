// Relative imports of sub modules
use common::client;
mod common;

#[test]
fn qa_rollout_flag() {
    let client = client();

    let flag_key = "qa_rollout";
    let decide_options = Vec::new();

    // The flag should be off for `user1 `
    let user_context = client.create_user_context("user1");
    let decision = user_context.decide(flag_key, &decide_options);
    assert!(!decision.enabled());
    assert_eq!(decision.variation_key(), "off");

    // The flag should be on for `user3 `
    let user_context = client.create_user_context("user3");
    let decision = user_context.decide(flag_key, &decide_options);
    assert!(decision.enabled());
    assert_eq!(decision.variation_key(), "on");
}

#[test]
fn invalid_flag() {
    let client = client();

    let flag_key = "this_flag_does_not_exist";
    let decide_options = Vec::new();

    // An invalid flag should always be disabled
    let user_context = client.create_user_context("_");
    let decision = user_context.decide(flag_key, &decide_options);
    assert!(!decision.enabled());
    assert_eq!(decision.variation_key(), "off");
}

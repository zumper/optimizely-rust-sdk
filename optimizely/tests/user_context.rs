// Imports from Optimizely crate
use optimizely::client::AttributeValue;
use optimizely::user_attributes;

// Relative imports of sub modules
use common::setup;
use serde_json::{json, Number};
mod common;

#[test]
fn user_context_set_attribute() {
    let ctx = setup();

    // Create user context without attributes
    let mut user_context = ctx.client.create_user_context("user123");

    // Override attributes on existing user context
    user_context.set_attribute("is_employee", true);
    user_context.set_attribute("app_version", "1.3.2");
    user_context.set_attribute("count", 47);
    user_context.set_attribute("nothing", AttributeValue::Null);

    // Attributes should be equal to expected
    assert!(user_context
        .attributes()
        .get("is_employee")
        .unwrap()
        .as_bool()
        .unwrap());
    assert_eq!(
        user_context
            .attributes()
            .get("app_version")
            .unwrap()
            .as_str()
            .unwrap(),
        "1.3.2"
    );
    assert_eq!(
        user_context
            .attributes()
            .get("count")
            .unwrap()
            .as_number()
            .unwrap(),
        &Number::from(47)
    );
    assert_eq!(
        json!(user_context.attributes()).to_string(),
        "{\"app_version\":\"1.3.2\",\"count\":47,\"is_employee\":true,\"nothing\":null}"
    );
}

#[test]
fn user_context_with_attributes() {
    let ctx = setup();

    // Create user context with given attributes
    let user_context = ctx.client.create_user_context_with_attributes(
        "user123",
        user_attributes! {
            "is_employee" => true,
            "app_version" => "1.3.2",
        },
    );

    // Attributes should be equal to expected
    assert!(user_context
        .attributes()
        .get("is_employee")
        .unwrap()
        .as_bool()
        .unwrap());
    assert_eq!(
        user_context
            .attributes()
            .get("app_version")
            .unwrap()
            .as_str()
            .unwrap(),
        "1.3.2"
    );
}

#[test]
#[cfg(feature = "online")]
fn user_context_track_event() {
    let ctx = setup();

    // Create user context with given attributes
    let user_context = ctx.client.create_user_context("user123");

    // Send a conversion event
    user_context.track_event("purchase");

    // Assert that exactly one event is dispatched
    assert_eq!(ctx.event_list.borrow().len(), 1);
}

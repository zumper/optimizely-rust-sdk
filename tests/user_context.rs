// Imports from Optimizely crate
use optimizely::user_attributes;
use optimizely::user_context::UserAttributes;

// Relative imports of sub modules
use common::setup;
mod common;

#[test]
fn user_context_attributes() {
    let ctx = setup();
    let mut user_context = ctx.client.create_user_context("user123");

    user_context.set_attribute("is_employee", "true");
    user_context.set_attribute("app_version", "1.3.2");

    // TODO: verify whether non-existing attributes should be returned
    let expected = user_attributes! {
        "is_employee" => "true",
        "app_version" => "1.3.2",
    };

    // Attributes should be equal to expected
    assert_eq!(*user_context.attributes(), expected);
}

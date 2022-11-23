// Imports from this crate
use optimizely::user_attributes;
use optimizely::user_context::UserAttributes;

// Relative imports of sub modules
use common::get_client;
mod common;

#[test]
fn user_context_attributes() {
    let client = get_client();
    let mut user_context = client.create_user_context("user123");

    user_context.set_attribute("is_employee", "true");
    user_context.set_attribute("app_version", "1.3.2");

    // TODO: verify whether non-existing attributes should be returned
    let expected = user_attributes! {
        "is_employee" => "true",
        "app_version" => "1.3.2",
    };

    // Attributes should be equal to expected
    assert_eq!(*user_context.get_attributes(), expected);
}

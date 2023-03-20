// External imports
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Serialize};

// Imports from super
use super::EventError;

// Relative imports of sub modules
use decision::Decision;
use event::Event;
use visitor::Visitor;
use snapshot::Snapshot;

mod decision;
mod event;
mod snapshot;
mod visitor;

// Information about the API endpoint
const ENDPOINT_URL: &str = "https://logx.optimizely.com/v1/events";
const CONTENT_TYPE_KEY: &str = "content-type";
const CONTENT_TYPE_VALUE: &str = "application/json";

// Information regarding the SDK client
const CLIENT_NAME: &str = "rust-sdk";
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Serialize)]
pub struct Payload<'a> {
    account_id: String,
    visitors: Vec<Visitor>,
    enrich_decisions: bool,
    anonymize_ip: bool,
    client_name: &'a str,
    client_version: &'a str,
}

impl Payload<'_> {
    pub fn new(account_id: String) -> Payload<'static> {
        Payload {
            account_id,
            visitors: Vec::<Visitor>::new(),
            enrich_decisions: true,
            anonymize_ip: true,
            client_name: CLIENT_NAME,
            client_version: CLIENT_VERSION,
        }
    }

    pub fn add_decision(
        &mut self, visitor_id: String, campaign_id: String, experiment_id: String, variation_id: String,
    ) {
        log::debug!("Adding decision event to log payload");

        // Use a copy of campaign_id as entity_id
        let entity_id = campaign_id.clone();

        // TODO: look up visitor ID in existing list

        // Retrieve existing visitor or insert new one
        let mut visitor = Visitor::new(visitor_id);

        // Add decision to visitor
        visitor.add_decision(campaign_id, experiment_id, variation_id);

        // Add campaign_activated event
        visitor.add_event(entity_id, String::from("campaign_activated"));

        // Add to the list
        self.visitors.push(visitor);
    }

    pub fn send(self) -> Result<(), EventError> {

        // Convert to JSON document and dump as String
        let body = serde_json::to_string(&self).unwrap();

        // Make POST request
        ureq::post(ENDPOINT_URL)
            .set(CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE)
            .send_string(&body)
            .into_report()
            .change_context(EventError::FailedRequest)?;

        Ok(())
    }
}

// External imports
use json::JsonValue;
use std::collections::HashMap;

// Relative imports of sub modules
use decision::Decision;
use event::Event;
use visitor::Visitor;

mod decision;
mod event;
mod visitor;

// Information about the API endpoint
const ENDPOINT_URL: &str = "https://logx.optimizely.com/v1/events";
const CONTENT_TYPE_KEY: &str = "content-type";
const CONTENT_TYPE_VALUE: &str = "application/json";

// Information regarding the SDK client
const CLIENT_NAME: &str = "rust-sdk";
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Payload {
    account_id: String,
    visitors: HashMap<String, Visitor>,
}

impl Payload {
    pub fn new(account_id: String) -> Payload {
        let visitors = HashMap::<String, Visitor>::new();

        Payload { account_id, visitors }
    }

    pub fn add_decision(
        &mut self,
        visitor_id: String,
        campaign_id: String,
        experiment_id: String,
        variation_id: String,
    ) {
        log::debug!("Adding decision event to log payload");

        // Use a copy of visitor id as the key
        let key = visitor_id.clone();

        // Use a copy of campaign_id as entity_id
        let entity_id = campaign_id.clone();

        // Retrieve existing visitor or insert new one
        let visitor = self.visitors.entry(key).or_insert(Visitor::new(visitor_id));

        // Add decision to visitor
        visitor.add_decision(campaign_id, experiment_id, variation_id);

        // Add campaign_activated event
        visitor.add_event(entity_id, String::from("campaign_activated"));
    }

    pub fn as_json(self) -> JsonValue {
        let visitors = self
            .visitors
            .into_iter()
            .map(|(_, visitor)| visitor.as_json())
            .collect::<Vec<_>>();

        json::object! {
            "account_id": self.account_id,
            "visitors": visitors,
            "enrich_decisions": true,
            "anonymize_ip": true,
            "client_name": CLIENT_NAME,
            "client_version": CLIENT_VERSION,
        }
    }

    pub fn send(self) {
        log::debug!("Sending log payload to Optimizely");

        // Convert to JSON document and dump as String
        let body = self.as_json().dump();

        // Make POST request
        match ureq::post(ENDPOINT_URL)
            .set(CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE)
            .send_string(&body)
        {
            Ok(_) => {
                // TODO: process response
                // TODO: include some data in log message
                log::info!("Log payload succesfully sent to Optimizely");
            }
            Err(_) => {
                // TODO: process error
                log::error!("Error while sending log payload");
            }
        }
    }
}

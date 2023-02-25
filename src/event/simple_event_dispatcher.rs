
// External imports
use anyhow::Result;
use std::time::SystemTime;
use std::rc::Rc;

// Imports from parent
use super::super::datafile::{Experiment, Variation};
use super::super::UserContext;

// Information regarding the SDK client
const CLIENT_NAME: &str = "rust-sdk";
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct SimpleEventDispatcher {
    account_id: String,
}

impl SimpleEventDispatcher {
    pub fn new(account_id: String) -> SimpleEventDispatcher {
        SimpleEventDispatcher {
            account_id
        }
    }

    pub fn send_decision(&self, user_context: &UserContext, experiment: &Experiment, variation: Rc<Variation>) -> Result<()> {
        // Get timestamp as milliseconds since the epoch
        // NOTE: Convert to u64 as json::object does not support u128
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis() as u64;

        // Decision object
        let decision = json::object! {
            "campaign_id": experiment.campaign_id().to_owned(),
            "experiment_id": experiment.id().to_owned(),
            "variation_id": variation.id().to_owned(),
            "is_campaign_holdback": false,
        };

        // Event object
        let event = json::object! {
            "entity_id": experiment.campaign_id().to_owned(),
            "type": "campaign_activated",
            "timestamp": timestamp,
            "uuid": 1,
        };

        // Snapshot object
        let snapshot = json::object! {
            "decisions": [decision],
            "events": [event],
        };

        // Visitor object
        let visitor = json::object! {
            "visitor_id": user_context.user_id().to_owned(),
            "snapshots": [snapshot],
        };

        // POST request payload
        let payload = json::object! {
            "account_id": self.account_id.clone(),
            "visitors": [visitor],
            "enrich_decisions": true,
            "anonymize_ip": true,
            "client_name": CLIENT_NAME,
            "client_version": CLIENT_VERSION,
        };

        // Make POST request
        let response = ureq::post("https://logx.optimizely.com/v1/events")
            .set("content-type", "application/json")
            .send_string(&payload.dump())?;

        // Debug
        dbg!(response);

        Ok(())
    }
}
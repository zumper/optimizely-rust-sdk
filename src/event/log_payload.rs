// External imports
use json::JsonValue;
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

// Information about the API endpoint
const ENDPOINT_URL: &str = "https://logx.optimizely.com/v1/events";
const CONTENT_TYPE_KEY: &str= "content-type";
const CONTENT_TYPE_VALUE: &str= "application/json";

// Information regarding the SDK client
const CLIENT_NAME: &str = "rust-sdk";
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct LogDecision {
    campaign_id: String,
    experiment_id: String,
    variation_id: String,
    is_campaign_holdback: bool,
}

impl LogDecision {
    fn new(campaign_id: String, experiment_id: String, variation_id: String) -> LogDecision {
        LogDecision {
            campaign_id,
            experiment_id,
            variation_id,
            is_campaign_holdback: false,
        }
    }

    fn as_json(self) -> JsonValue {
        json::object! {
            "campaign_id": self.campaign_id,
            "experiment_id": self.experiment_id,
            "variation_id": self.variation_id,
            "is_campaign_holdback": self.is_campaign_holdback,
        }
    }
}

#[derive(Debug)]
struct LogEvent {
    uuid: String,
    timestamp: u64,
    entity_id: String,
    event_type: String,
    event_key: Option<String>,
    // tags
}

impl LogEvent {
    fn new(entity_id: String, event_type: String) -> LogEvent {
        // Generate new UUID
        let uuid = Uuid::new_v4().as_hyphenated().to_string();

        // Get timestamp as milliseconds since the epoch
        let timestamp: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                // NOTE: Convert to u64 as json::object does not support u128
                duration.as_millis() as u64
            }
            Err(_) => 0,
        };

        LogEvent {
            uuid,
            timestamp,
            entity_id,
            event_type,
            event_key: Option::None,
        }
    }

    fn as_json(self) -> JsonValue {
        // TODO: add these for custom events
        drop(self.event_key);

        json::object! {
            "entity_id": self.entity_id,
            "type": self.event_type,
            "timestamp": self.timestamp,
            "uuid": self.uuid,
        }
    }
}

#[derive(Debug)]
struct LogVisitor {
    visitor_id: String,
    // attributes
    decisions: Vec<LogDecision>,
    events: Vec<LogEvent>,
}

impl LogVisitor {
    fn new(visitor_id: String) -> LogVisitor {
        LogVisitor {
            visitor_id,
            decisions: vec![],
            events: vec![],
        }
    }

    fn add_decision(&mut self, campaign_id: String, experiment_id: String, variation_id: String) {
        self.decisions
            .push(LogDecision::new(campaign_id, experiment_id, variation_id));
    }

    fn add_event(&mut self, entity_id: String, event_type: String) {
        self.events.push(LogEvent::new(entity_id, event_type));
    }

    fn as_json(self) -> JsonValue {
        let decisions = self
            .decisions
            .into_iter()
            .map(|decision| decision.as_json())
            .collect::<Vec<_>>();

        let events = self
            .events
            .into_iter()
            .map(|event| event.as_json())
            .collect::<Vec<_>>();

        let snapshot = json::object! {
            "decisions": decisions,
            "events": events,
        };

        json::object! {
            "visitor_id": self.visitor_id,
            "snapshots": [snapshot],
        }
    }
}

#[derive(Debug)]
pub(super) struct LogPayload {
    account_id: String,
    visitors: HashMap<String, LogVisitor>,
}

impl LogPayload {
    pub fn new(account_id: String) -> LogPayload {
        let visitors = HashMap::<String, LogVisitor>::new();

        LogPayload { account_id, visitors }
    }

    pub fn add_decision(
        &mut self,
        visitor_id: String,
        campaign_id: String,
        experiment_id: String,
        variation_id: String,
    ) {
        // Use a copy of visitor id as the key
        let key = visitor_id.clone();

        // Use a copy of campaign_id as entity_id
        let entity_id = campaign_id.clone();

        // Retrieve existing visitor or insert new one
        let visitor = self
            .visitors
            .entry(key)
            .or_insert(LogVisitor::new(visitor_id));

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
        // Convert to JSON document and dump as String
        let body = self.as_json().dump();

        // Make POST request
        match ureq::post(ENDPOINT_URL)
            .set(CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE)
            .send_string(&body)
        {
            Ok(response) => {
                // TODO: process response
                dbg!(response);
            }
            Err(error) => {
                // TODO: process error
                dbg!(error);
            }
        }

    }
}

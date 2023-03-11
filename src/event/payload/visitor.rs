// External imports
use serde_json::{json, Value as JsonValue};

// Imports from crate
use crate::event::payload::{Decision, Event};

pub struct Visitor {
    visitor_id: String,
    // attributes
    decisions: Vec<Decision>,
    events: Vec<Event>,
}

impl Visitor {
    pub fn new(visitor_id: String) -> Visitor {
        Visitor {
            visitor_id,
            decisions: vec![],
            events: vec![],
        }
    }

    pub fn add_decision(&mut self, campaign_id: String, experiment_id: String, variation_id: String) {
        self.decisions
            .push(Decision::new(campaign_id, experiment_id, variation_id));
    }

    pub fn add_event(&mut self, entity_id: String, event_type: String) {
        self.events.push(Event::new(entity_id, event_type));
    }

    pub fn as_json(self) -> JsonValue {
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

        let snapshot = json!({
            "decisions": decisions,
            "events": events,
        });

        json!({
            "visitor_id": self.visitor_id,
            "snapshots": [snapshot],
        })
    }
}

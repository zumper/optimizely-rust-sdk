// External imports
use serde::Serialize;

// Imports from super
use super::{Decision, Event};

#[derive(Serialize, Default)]
pub struct Snapshot {
    decisions: Vec<Decision>,
    events: Vec<Event>,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        Snapshot::default()
    }

    pub fn add_decision(&mut self, campaign_id: String, experiment_id: String, variation_id: String) {
        let decision = Decision::new(campaign_id, experiment_id, variation_id);
        self.decisions.push(decision);
    }

    pub fn add_event(&mut self, entity_id: String, event_key: String) {
        let event = Event::new(entity_id, event_key);
        self.events.push(event);
    }
}

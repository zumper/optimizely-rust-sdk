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
        self.decisions
            .push(Decision::new(campaign_id, experiment_id, variation_id));
    }

    pub fn add_event(&mut self, entity_id: String, event_type: String) {
        self.events.push(Event::new(entity_id, event_type));
    }
}

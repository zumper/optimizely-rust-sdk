// External imports
use serde::Serialize;

// Imports from crate
use crate::event::payload::Snapshot;

#[derive(Serialize)]
pub struct Visitor {
    visitor_id: String,
    // attributes
    snapshots: [Snapshot; 1],
}

impl Visitor {
    pub fn new(visitor_id: String) -> Visitor {
        Visitor {
            visitor_id,
            snapshots: [Snapshot::new()],
        }
    }

    pub fn add_decision(&mut self, campaign_id: String, experiment_id: String, variation_id: String) {
        self.snapshots[0].add_decision(campaign_id, experiment_id, variation_id);
    }

    pub fn add_event(&mut self, entity_id: String, event_type: String) {
        self.snapshots[0].add_event(entity_id, event_type);
    }
}

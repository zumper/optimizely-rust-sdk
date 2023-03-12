// Imports from super
use super::{Event, EventDispatcher, Payload};

pub struct SimpleEventDispatcher {}

impl SimpleEventDispatcher {
    pub fn new() -> SimpleEventDispatcher {
        SimpleEventDispatcher {}
    }
}

impl EventDispatcher for SimpleEventDispatcher {
    fn send_event(&self, event: Event) {
        match event {
            Event::Decision {
                account_id,
                user_id,
                campaign_id,
                experiment_id,
                variation_id,
            } => {
                // Generate a new payload
                let mut payload = Payload::new(account_id);

                // Add single decision
                payload.add_decision(user_id, campaign_id, experiment_id, variation_id);

                // And send
                payload.send();
            }
            _ => {
                // TODO:
            }
        }
    }
}

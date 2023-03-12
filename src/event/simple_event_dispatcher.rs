// Imports from super
use super::{Event, EventDispatcher, Payload};

/// Implementation of the EventDisptacher trait that makes an HTTP request for every event
///
/// ```
/// use optimizely::event::{Event, SimpleEventDispatcher, EventDispatcher};
///
/// let account_id = "21537940595";
/// let user_id = "user0";
/// let campaign_id = "9300000133039";
/// let experiment_id = "9300000169122";
/// let variation_id = "87757";
///
/// let event = Event::decision(account_id, user_id, campaign_id, experiment_id, variation_id);
///
/// let dispatcher = SimpleEventDispatcher::new();
/// dispatcher.send_event(event);
/// ```
pub struct SimpleEventDispatcher {}

impl SimpleEventDispatcher {
    /// Constructor for a new simple event dispatcher
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

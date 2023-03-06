// External imports
use std::rc::Rc;

// Imports from crate
use crate::datafile::{Experiment, Variation};
use crate::event::{log::Payload, EventDispatcher};
use crate::UserContext;

pub struct SimpleEventDispatcher {}

impl SimpleEventDispatcher {
    pub fn new() -> SimpleEventDispatcher {
        SimpleEventDispatcher {}
    }
}

impl EventDispatcher for SimpleEventDispatcher {
    fn send_decision(&self, user_context: &UserContext, experiment: &Experiment, variation: Rc<Variation>) {
        // Generate a payload for a single decision event
        let mut payload = Payload::new(user_context.client().account_id().to_owned());

        // Add single decision
        payload.add_decision(
            user_context.user_id().to_owned(),
            experiment.campaign_id().to_owned(),
            experiment.id().to_owned(),
            variation.id().to_owned(),
        );

        payload.send();
    }
}

// External imports
use std::rc::Rc;

// Imports from crate
use crate::datafile::{Experiment, Variation};
use crate::event::{EventDispatcher, LogPayload};
use crate::UserContext;

pub struct SimpleEventDispatcher {
    account_id: String,
}

impl SimpleEventDispatcher {
    pub fn new(account_id: String) -> SimpleEventDispatcher {
        SimpleEventDispatcher { account_id }
    }
}

impl EventDispatcher for SimpleEventDispatcher {
    fn send_decision(&self, user_context: &UserContext, experiment: &Experiment, variation: Rc<Variation>) {
        // Generate a payload for a single decision event
        let mut payload = LogPayload::new(self.account_id.to_owned());
        payload.add_decision(
            user_context.user_id().to_owned(),
            experiment.campaign_id().to_owned(),
            experiment.id().to_owned(),
            variation.id().to_owned(),
        );

        payload.send();
    }
}

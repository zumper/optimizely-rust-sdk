// External imports
use std::rc::Rc;

// Imports from crate
use crate::datafile::{Experiment, Variation};
use crate::UserContext;

// TODO: adjust the Trait interface to not expose Experiment and Variation
pub trait EventDispatcher {
    fn send_decision(&self, user_context: &UserContext, experiment: &Experiment, variation: Rc<Variation>);
}

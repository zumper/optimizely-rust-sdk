// External imports
use std::rc::Rc;

// Imports from crate
use crate::datafile::{Experiment, Variation};
use crate::UserContext;

pub trait EventDispatcher {
    fn send_decision(&self, user_context: &UserContext, experiment: &Experiment, variation: Rc<Variation>);
}

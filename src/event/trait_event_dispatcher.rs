// External imports
use std::rc::Rc;

// Imports from parent
use super::super::datafile::{Experiment, Variation};
use super::super::UserContext;

pub trait EventDispatcher {
    fn send_decision(&self, user_context: &UserContext, experiment: &Experiment, variation: Rc<Variation>);
}

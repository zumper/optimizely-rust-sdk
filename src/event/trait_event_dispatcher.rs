// Imports from super
use super::Event;

// TODO: adjust the Trait interface to not expose Experiment and Variation
pub trait EventDispatcher {
    fn send_event(&self, event: Event);
}

// Imports from super
use super::Event;

/// Trait for sending events to Optimizely Event API
///
/// It is possible to make a custom event disptacher by implementing this trait
/// ```text
/// TODO: example that stores in vector
/// ```
pub trait EventDispatcher {
    /// Send event to destination
    fn send_event(&self, event: Event);
}

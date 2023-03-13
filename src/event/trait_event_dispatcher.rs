// Imports from super
use super::Event;

/// Trait for sending events to Optimizely Event API
///
/// It is possible to make a custom event disptacher by implementing this trait
/// ```
/// use std::cell::RefCell;
/// use optimizely::event::{Event, EventDispatcher};
/// #
/// # // Create some example IDs
/// # let account_id = "21537940595";
/// # let user_id = "user0";
/// # let campaign_id = "9300000133039";
/// # let experiment_id = "9300000169122";
/// # let variation_id = "87757";
/// #
/// # // Create new event from above IDs
/// # let event = Event::decision(account_id, user_id, campaign_id, experiment_id, variation_id);
///
/// // Struct that will store events instead of sending them
/// #[derive(Default)]
/// struct EventStore {
///     list: RefCell<Vec<Event>>
/// }
///
/// // Easy way to get the length of the list inside
/// impl EventStore {
///     fn size(&self) -> usize {
///         self.list.borrow().len()
///     }
/// }
///
/// // Implementation of the EventDispatcher trait
/// impl EventDispatcher for EventStore {
///     fn send_event(&self, event: Event) {
///         self.list.borrow_mut().push(event);
///     }
/// }
///
/// // Initialize an empty event store
/// let event_store = EventStore::default();
/// assert_eq!(event_store.size(), 0);
///
/// // Send one event
/// event_store.send_event(event);
/// assert_eq!(event_store.size(), 1);
/// ```
pub trait EventDispatcher {
    /// Send event to destination
    fn send_event(&self, event: Event);
}

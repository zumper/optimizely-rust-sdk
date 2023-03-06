// Relative imports of sub modules
pub use simple_event_dispatcher::SimpleEventDispatcher;
pub use batched_event_dispatcher::BatchedEventDispatcher;
pub use trait_event_dispatcher::EventDispatcher;

mod simple_event_dispatcher;
mod trait_event_dispatcher;
mod log;
mod batched_event_dispatcher;

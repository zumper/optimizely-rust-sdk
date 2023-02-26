// Relative imports of sub modules
pub use simple_event_dispatcher::SimpleEventDispatcher;
use log_payload::LogPayload;
pub use trait_event_dispatcher::EventDispatcher;

mod simple_event_dispatcher;
mod trait_event_dispatcher;
mod log_payload;

//! Event logging to Optimizely Event API

// Relative imports of sub modules
pub use simple_event_dispatcher::SimpleEventDispatcher;
pub use batched_event_dispatcher::BatchedEventDispatcher;
pub use trait_event_dispatcher::EventDispatcher;
pub use error::EventError;
use payload::Payload;

mod simple_event_dispatcher;
mod trait_event_dispatcher;
mod payload;
mod batched_event_dispatcher;
mod error;

/// Representation of the events which can be dispatched to Optimizely Event API
///
/// An event can either be a decision or conversion.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Event {

    #[doc(hidden)]
    Decision{
        account_id: String,
        user_id: String,
        campaign_id: String,
        experiment_id: String,
        variation_id: String,
    },

    #[doc(hidden)]
    Conversion{
        account_id: String,
        user_id: String,
    },
}

impl Event {

    /// Constructor for a new decision event
    ///
    /// ```
    /// use optimizely::event::Event;
    ///
    /// // Create some example IDs
    /// let account_id = "21537940595";
    /// let user_id = "user0";
    /// let campaign_id = "9300000133039";
    /// let experiment_id = "9300000169122";
    /// let variation_id = "87757";
    ///
    /// // Create new event from above IDs
    /// let event = Event::decision(account_id, user_id, campaign_id, experiment_id, variation_id);
    /// ```
    pub fn decision(account_id: &str, user_id: &str, campaign_id: &str, experiment_id: &str, variation_id: &str) -> Event {
        Event::Decision {
            account_id: account_id.to_owned(),
            user_id: user_id.to_owned(),
            campaign_id: campaign_id.to_owned(),
            experiment_id: experiment_id.to_owned(),
            variation_id: variation_id.to_owned(),
        }
    }
}

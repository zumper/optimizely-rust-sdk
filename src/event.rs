//! Event logging to logx.ptimizely.com

// Relative imports of sub modules
pub use simple_event_dispatcher::SimpleEventDispatcher;
pub use batched_event_dispatcher::BatchedEventDispatcher;
pub use trait_event_dispatcher::EventDispatcher;
use payload::Payload;

mod simple_event_dispatcher;
mod trait_event_dispatcher;
mod payload;
mod batched_event_dispatcher;

// When sending an event it can either be a decision event or conversion event
// TODO: implement conversion event
#[allow(dead_code)]
pub enum Event {
    Decision{
        account_id: String,
        user_id: String,
        campaign_id: String,
        experiment_id: String,
        variation_id: String,
    },
    Conversion{
        account_id: String,
        user_id: String,
    },
}

impl Event {
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
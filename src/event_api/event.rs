/// Representation of the events which can be dispatched to Optimizely Event API
///
/// An event can either be a decision or conversion.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Event {
    #[doc(hidden)]
    Decision {
        account_id: String,
        user_id: String,
        campaign_id: String,
        experiment_id: String,
        variation_id: String,
    },

    #[doc(hidden)]
    Conversion { account_id: String, user_id: String },
}

impl Event {
    /// Constructor for a new decision event
    ///
    /// ```
    /// use optimizely::event_api::Event;
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
    pub fn decision<T: Into<String>>(
        account_id: T, user_id: T, campaign_id: T, experiment_id: T, variation_id: T,
    ) -> Event {
        Event::Decision {
            account_id: account_id.into(),
            user_id: user_id.into(),
            campaign_id: campaign_id.into(),
            experiment_id: experiment_id.into(),
            variation_id: variation_id.into(),
        }
    }
}

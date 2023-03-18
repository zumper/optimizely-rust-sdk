// External imports
use serde_json::{json, Value as JsonValue};

pub struct Decision {
    campaign_id: String,
    experiment_id: String,
    variation_id: String,
    is_campaign_holdback: bool,
}

impl Decision {
    pub fn new(campaign_id: String, experiment_id: String, variation_id: String) -> Decision {
        Decision {
            campaign_id,
            experiment_id,
            variation_id,
            is_campaign_holdback: false,
        }
    }

    pub fn into_json(self) -> JsonValue {
        json!({
            "campaign_id": self.campaign_id,
            "experiment_id": self.experiment_id,
            "variation_id": self.variation_id,
            "is_campaign_holdback": self.is_campaign_holdback,
        })
    }
}

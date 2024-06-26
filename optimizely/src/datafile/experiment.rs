// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// Imports from super
use super::{BooleanCondition, TrafficAllocation, Variation};

#[derive(Deserialize, Debug)]
pub struct Experiment {
    #[serde()]
    id: String,
    #[serde()]
    key: String,
    #[serde(rename = "audienceConditions")]
    audience_conditions: Option<BooleanCondition<String>>,
    #[serde(rename = "audienceIds")]
    audience_ids: Vec<String>,
    #[serde(rename = "layerId")]
    campaign_id: String,
    #[serde(rename = "trafficAllocation", deserialize_with = "TrafficAllocation::deserialize")]
    traffic_allocation: TrafficAllocation,
    #[serde(rename = "variations", deserialize_with = "Variation::deserialize")]
    variations: HashMap<String, Variation>,
}

impl Experiment {
    // Method to deserialize an array of Experiments into a Hashmap of Experiments
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Experiment>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for experiment in Vec::<Experiment>::deserialize(deserializer)? {
            map.insert(experiment.id.clone(), experiment);
        }
        Ok(map)
    }

    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[allow(dead_code)]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[allow(dead_code)]
    pub fn campaign_id(&self) -> &str {
        &self.campaign_id
    }

    #[allow(dead_code)]
    pub fn traffic_allocation(&self) -> &TrafficAllocation {
        &self.traffic_allocation
    }

    pub fn variation(&self, variation_id: &str) -> Option<&Variation> {
        self.variations.get(variation_id)
    }

    pub fn evaluate_audience_conditions<E>(&self, evaluator: &E) -> bool
    where
        E: Fn(&String) -> bool,
    {
        if let Some(conditions) = &self.audience_conditions {
            if conditions.is_empty() {
                return true;
            }
            return conditions.evaluate(evaluator);
        } else {
            let conditions = BooleanCondition::Or(
                self.audience_ids
                    .iter()
                    .map(|id| Box::new(BooleanCondition::Single(id.into())))
                    .collect(),
            );
            if conditions.is_empty() {
                return true;
            }
            return conditions.evaluate(evaluator);
        }
    }
}

// External imports
use anyhow::Result;
use json::JsonValue;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

// Imports from crate
use crate::datafile::{DatafileError, Variation};

#[derive(Debug)]
pub struct TrafficAllocation {
    ranges: BTreeMap<u32, Rc<Variation>>,
}

impl Default for TrafficAllocation {
    fn default() -> Self {
        TrafficAllocation {
            ranges: BTreeMap::default(),
        }
    }
}

impl TrafficAllocation {
    pub fn build(value: &mut JsonValue, variations: &mut HashMap<String, Rc<Variation>>) -> Result<TrafficAllocation> {
        // BUG: found an example datafile where entityId is an empty string, so return an empty traffic allocation
        if value["trafficAllocation"]
            .members()
            .any(|value| value["entityId"] == "")
        {
            return Ok(TrafficAllocation::default());
        }

        // A closure to return pairs of Variation and their end of range
        let get_allocation = |value: &mut JsonValue| -> Result<(u32, Rc<Variation>)> {
            // Get id as string
            let variation_id = string_field!(value, "entityId")?;

            // Get end of range as integer
            let end_of_range = u32_field!(value, "endOfRange")?;

            // Remove from hashmap to get an owned copy
            let variation = variations
                .get(&variation_id)
                .ok_or(DatafileError::InvalidVariationId(variation_id))?;

            // NOTE: the datafile might contain the same variation multiple times in the traffic allocation
            // Hence we clone a reference-counting pointer
            let variation = Rc::clone(variation);

            // Return as a tuple
            Ok((end_of_range, variation))
        };

        // Create a binary tree for efficient look ups
        let ranges: Vec<(u32, Rc<Variation>)> = list_field!(value, "trafficAllocation", get_allocation)?;
        let ranges: BTreeMap<u32, Rc<Variation>> = ranges.into_iter().collect();

        // Initialize struct and return result
        let traffic_allocation = TrafficAllocation { ranges };
        Ok(traffic_allocation)
    }

    pub fn get_variation_for_bucket(&self, bucket_value: u32) -> Option<Rc<Variation>> {
        // Use BTreeMap::range to find the variation in O(log(n))
        match self.ranges.range(bucket_value..).next() {
            None => None,
            Some((_, variation)) => {
                // Unwrap the variation from the tuple and clone a reference-counting pointer
                Some(Rc::clone(variation))
            }
        }
    }
}

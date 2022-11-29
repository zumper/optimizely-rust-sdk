// External imports
use anyhow::Result;
use json::JsonValue;
use std::collections::{BTreeMap, HashMap};

// Imports from parent
use super::{Variation, DatafileError};

#[derive(Debug)]
pub struct TrafficAllocation {
    pub ranges: BTreeMap<u32, Variation>,
}

impl TrafficAllocation {
    pub fn build(datafile: &mut JsonValue, variations: &mut HashMap<String, Variation>) -> Result<TrafficAllocation> {
        // A closure to return pairs of Variation and their end of range
        let get_allocation = |value: &mut JsonValue| -> Result<(u32, Variation)> {
            // Get id as string
            let variation_id = string_field!(value, "entityId")?;

            // Get end of range as integer
            let end_of_range = u32_field!(value, "endOfRange")?;

            // Remove from hashmap to get an owned copy
            // ERROR: datafile can reference the same variation multiple times
            let variation = variations
                .remove(&variation_id)
                .ok_or(DatafileError::InvalidVariationId(variation_id))?;

            // Return as a tuple
            Ok((end_of_range, variation))
        };

        // Create a binary tree for efficient look ups
        let ranges: Vec<(u32, Variation)> = list_field!(datafile, "trafficAllocation", get_allocation)?;
        let ranges: BTreeMap<u32, Variation> = ranges.into_iter().collect();

        // Initialize struct and return result
        let traffic_allocation = TrafficAllocation { ranges };
        Ok(traffic_allocation)
    }
}

// External imports
use error_stack::{Report, Result};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

// Imports from super
use super::{DatafileError, Json, Variation};

#[derive(Debug, Default)]
pub struct TrafficAllocation {
    ranges: BTreeMap<u64, Rc<Variation>>,
}

impl TrafficAllocation {
    pub(crate) fn build(
        json: &mut Json,
        variations: &mut HashMap<String, Rc<Variation>>,
    ) -> Result<TrafficAllocation, DatafileError> {
        // Create a binary tree for efficient look ups
        let ranges = json
            .get("trafficAllocation")?
            .as_array()?
            .map(|mut json| {
                // Get variation_id as String
                let variation_id = json.get("entityId")?.as_string()?;

                // Get end_of_range as u64
                let end_of_range = json.get("endOfRange")?.as_integer()?;

                // Remove from hashmap to get an owned copy
                let variation = variations
                    .get(&variation_id)
                    .ok_or_else(|| Report::new(DatafileError::InvalidVariationId(variation_id)))?;

                // NOTE: the datafile might contain the same variation multiple times in the traffic allocation
                // Hence we clone a reference-counting pointer
                let variation = Rc::clone(variation);

                // Return as a tuple
                Ok((end_of_range, variation))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<BTreeMap<_, _>>();

        // Initialize struct and return result
        Ok(TrafficAllocation { ranges })
    }

    pub(crate) fn get_variation_for_bucket(&self, bucket_value: u64) -> Option<Rc<Variation>> {
        // Use BTreeMap::range to find the variation in O(log(n))
        self.ranges
            .range(bucket_value..)
            .next()
            .map(|(_, variation)| Rc::clone(variation))
    }
}

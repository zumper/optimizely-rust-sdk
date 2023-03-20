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
    pub(crate) fn new(ranges: BTreeMap<u64, Rc<Variation>>) -> TrafficAllocation {
        TrafficAllocation { ranges }
    }

    pub(crate) fn build(
        json: &mut Json, variations: &mut HashMap<String, Rc<Variation>>,
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
        Ok(TrafficAllocation::new(ranges))
    }

    pub(crate) fn get_variation_for_bucket(&self, bucket_value: u64) -> Option<&Variation> {
        // Use BTreeMap::range to find the variation in O(log(n))
        self.ranges
            .range(bucket_value..)
            .next()
            .map(|(_, variation)| variation.as_ref())
    }
}

/// Macro to create UserAttributes
/// Currently only used for testing
#[cfg(test)]
macro_rules! traffic_allocation {
    { $( $end_of_range: literal => $variation: expr),* $(,)?} => {
        {
            let mut ranges = BTreeMap::<u64, Rc<Variation>>::new();

            $(
                ranges.insert($end_of_range, Rc::clone($variation));
            )*

            TrafficAllocation::new(ranges)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_variation_for_bucket() {
        let variation_a = Rc::new(Variation::new("100", "A", true));
        let variation_b = Rc::new(Variation::new("101", "B", true));
        let variation_c = Rc::new(Variation::new("102", "C", true));

        let traffic_allocation = traffic_allocation! {
            3_333 => &variation_a,
            6_666 => &variation_b,
            10_000 => &variation_c,
        };

        assert_eq!(traffic_allocation.get_variation_for_bucket(0), Some(variation_a.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(1_000), Some(variation_a.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(2_000), Some(variation_a.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(3_000), Some(variation_a.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(4_000), Some(variation_b.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(5_000), Some(variation_b.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(6_000), Some(variation_b.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(7_000), Some(variation_c.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(8_000), Some(variation_c.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(9_000), Some(variation_c.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(10_000), Some(variation_c.as_ref()));
        assert_eq!(traffic_allocation.get_variation_for_bucket(11_000), None);
        assert_eq!(traffic_allocation.get_variation_for_bucket(99_000), None);
    }
}

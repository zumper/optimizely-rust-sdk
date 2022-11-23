// External imports
use std::collections::BTreeMap;

// Imports from parent
use super::Variation;

pub struct TrafficAllocation {
    pub ranges: BTreeMap<u32, Variation>,
}

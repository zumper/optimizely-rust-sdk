//! Everything related to parsing the Optimizely datafile

pub use error::DatafileError;
pub use feature_flag::FeatureFlag;
pub use rollout::Rollout;

mod error;
mod feature_flag;
mod rollout;

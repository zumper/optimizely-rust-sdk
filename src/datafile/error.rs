// External imports
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DatafileError {
    #[error("Missing field in datafile: {0:?}")]
    MissingField(String),
    #[error("Revision is not parsable as integer")]
    InvalidRevision,
    #[error("Rollout ID does not exist: {0:?}")]
    InvalidRolloutId(String),
}

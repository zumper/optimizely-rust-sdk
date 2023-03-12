// External imports
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DatafileError {
    #[error("JSON can not be parsed")]
    InvalidJson,
    #[error("Key {0:?} not found")]
    KeyNotFound(String),
    #[error("Element did not have type {0:?}")]
    InvalidType(String),
    #[error("Revision {0:?} is not parsable as integer")]
    InvalidRevision(String),
    #[error("Rollout ID {0:?} does not exist")]
    InvalidRolloutId(String),
    #[error("Variation ID {0:?} does not exist")]
    InvalidVariationId(String),
}

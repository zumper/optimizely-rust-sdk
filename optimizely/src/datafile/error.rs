// External imports
use thiserror::Error;

/// This type represents all possible errors that can occur when parsing the datafile
#[derive(Error, Debug, PartialEq)]
pub enum DatafileError {
    #[doc(hidden)]
    #[error("JSON can not be parsed")]
    InvalidJson,
    #[doc(hidden)]
    #[error("Key {0:?} not found")]
    KeyNotFound(String),
    #[doc(hidden)]
    #[error("Element did not have type {0:?}")]
    InvalidType(String),
    #[doc(hidden)]
    #[error("Revision {0:?} is not parsable as integer")]
    InvalidRevision(String),
    #[doc(hidden)]
    #[error("Rollout ID {0:?} does not exist")]
    InvalidRolloutId(String),
    #[doc(hidden)]
    #[error("Variation ID {0:?} does not exist")]
    InvalidVariationId(String),
}

// External imports
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DatafileError {
    #[error("Failed to make request to cdn.optimizely.com")]
    FailedRequest,
    #[error("Failed to decode response from cdn.optimizely.com")]
    FailedResponse,
    #[error("Failed to open local datafile")]
    FailedFileOpen,
    #[error("Failed to read from local datafile")]
    FailedFileRead,
    #[error("Missing field in datafile: {0:?}")]
    MissingField(String),
    #[error("Revision is not parsable as integer")]
    InvalidRevision,
    #[error("Rollout ID does not exist: {0:?}")]
    InvalidRolloutId(String),
    #[error("Experiment ID is missing")]
    MissingExperimentId,
    #[error("Variation ID does not exist: {0:?}")]
    InvalidVariationId(String),
}

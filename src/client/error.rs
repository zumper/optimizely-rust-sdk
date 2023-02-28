// External imports
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ClientError {
    #[error("Cannot build Client without given Datafile")]
    DatafileMissing,
    #[error("Failed to make request to cdn.optimizely.com")]
    FailedRequest,
    #[error("Failed to decode response from cdn.optimizely.com")]
    FailedResponse,
    #[error("Failed to open local datafile")]
    FailedFileOpen,
    #[error("Failed to read from local datafile")]
    FailedFileRead,
}

// External imports
use thiserror::Error;

/// This type represents all possible errors that can occur when initializing the client
#[derive(Error, Debug, PartialEq)]
pub enum ClientError {
    #[doc(hidden)]
    #[error("Cannot build Client without given Datafile")]
    DatafileMissing,
    #[doc(hidden)]
    #[error("Failed to make request to cdn.optimizely.com")]
    FailedRequest,
    #[doc(hidden)]
    #[error("Failed to decode response from cdn.optimizely.com")]
    FailedResponse,
    #[doc(hidden)]
    #[error("Failed to open local datafile")]
    FailedFileOpen,
    #[doc(hidden)]
    #[error("Failed to read from local datafile")]
    FailedFileRead,
    #[doc(hidden)]
    #[error("Invalid Datafile")]
    InvalidDatafile,
}

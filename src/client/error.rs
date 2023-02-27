// External imports
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ClientError {
    #[error("Cannot build Client without given Datafile")]
    DatafileMissing,
}

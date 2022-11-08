// External imports
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum DatafileError {
    MissingField(String),
    InvalidRevision,
    InvalidRolloutId(String),
}

impl Display for DatafileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DatafileError::MissingField(field) => {
                write!(f, "Missing field in datafile: {:?}", field)
            }
            DatafileError::InvalidRevision => {
                write!(f, "Revision is not parsable as integer")
            }
            DatafileError::InvalidRolloutId(id) => {
                write!(f, "Rollout ID does not exist: {:?}", id)
            }
        }
    }
}

impl Error for DatafileError {}

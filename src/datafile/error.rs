// External imports
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum DatafileError<'a> {
    MissingField(&'a str),
    InvalidRevision,
}

impl Display for DatafileError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DatafileError::MissingField(field) => {
                write!(f, "Missing field in datafile: {:?}", field)
            }
            DatafileError::InvalidRevision => {
                write!(f, "Revision is not parsable as integer")
            }
        }
    }
}

impl Error for DatafileError<'_> {}

// External imports
use std::error::Error;
// Imports from parent
use super::UserContext;
// Relative imports of sub modules
pub use datafile_error::DatafileError;
mod datafile_error;

#[derive(Debug)]
pub struct Client {
    pub account_id: String,
    pub revision: u32,
}

impl Client {
    pub fn build(datafile: &str) -> Result<Client, Box<dyn Error>> {
        // Parse datafile as JSON
        let datafile = json::parse(datafile)?;

        // TODO: read out parsed JSON

        // Get account id as string
        let account_id = datafile["accountId"]
            .as_str()
            .ok_or(DatafileError::MissingField("accountId"))?
            .to_owned();

        // Get account id as string
        let revision = datafile["revision"]
            .as_str()
            .ok_or(DatafileError::MissingField("revision"))?
            .parse::<u32>()
            .map_err(|_| DatafileError::InvalidRevision)?;

        Ok(Client {
            account_id,
            revision,
        })
    }

    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext {
        UserContext::new(user_id)
    }
}

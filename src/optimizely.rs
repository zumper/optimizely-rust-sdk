// External imports
use std::error::Error;
// Imports from parent
use super::UserContext;

#[derive(Debug)]
pub struct Optimizely {
    pub initialized: bool,
}

impl Optimizely {
    pub fn build(datafile: &str) -> Result<Optimizely, Box<dyn Error>> {
        let parsed = json::parse(datafile)?;

        // TODO: read out parsed JSON
        drop(parsed);

        Ok(Optimizely { initialized: true })
    }

    pub fn create_user_context(&self, user_id: &str) -> UserContext {
        UserContext::new(user_id)
    }
}

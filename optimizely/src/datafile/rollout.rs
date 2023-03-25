// External imports
use error_stack::Result;

// Imports from super
use super::{DatafileError, Experiment, Json};

#[derive(Debug)]
pub struct Rollout {
    id: String,
    experiments: Vec<Experiment>,
}

impl Rollout {
    pub(crate) fn new<T: Into<String>>(id: T, experiments: Vec<Experiment>) -> Rollout {
        Rollout {
            id: id.into(),
            experiments,
        }
    }

    pub(crate) fn build(json: &mut Json) -> Result<Rollout, DatafileError> {
        let id = json.get("id")?.as_string()?;

        let experiments = json
            .get("experiments")?
            .as_array()?
            .map(|mut json| Experiment::build(&mut json))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Rollout::new(id, experiments))
    }

    pub fn experiments(&self) -> &Vec<Experiment> {
        &self.experiments
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

// External imports
use error_stack::Result;

// Imports from super
use super::{Context, DatafileError, Experiment};

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

    pub(crate) fn build(context: &mut Context) -> Result<Rollout, DatafileError> {
        let id = context.get("id")?.as_string()?;

        let experiments = context
            .get("experiments")?
            .as_array()?
            .map(|mut context| Experiment::build(&mut context))
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

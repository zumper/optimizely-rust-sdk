// External imports
use error_stack::{IntoReport, Report, Result, ResultExt};

// Imports from super
use super::DatafileError;

static TYPE_STRING: &str = "str";
static TYPE_INTEGER: &str = "u64";
static TYPE_BOOLEAN: &str = "bool";
static TYPE_ARRAY: &str = "array";

pub(crate) struct Context(serde_json::Value);

impl Context {
    pub(crate) fn build(content: &str) -> Result<Context, DatafileError> {
        // Parse content as JSON
        let value = serde_json::from_str(content)
            .into_report()
            .change_context(DatafileError::InvalidJson)?;

        Ok(Context(value))
    }

    pub(crate) fn get(&mut self, key: &str) -> Result<Context, DatafileError> {
        let value = self
            .0
            .get_mut(key)
            .ok_or_else(|| {
                // Unable to find property
                Report::new(DatafileError::KeyNotFound(key.into()))
            })?
            .take();

        Ok(Context(value))
    }

    pub(crate) fn as_string(&self) -> Result<String, DatafileError> {
        let value = self.0.as_str().ok_or_else(|| {
            // Unable to read property as a string
            Report::new(DatafileError::InvalidType(TYPE_STRING.into()))
        })?;

        // Create owned copy
        let value = value.to_owned();

        Ok(value)
    }

    pub(crate) fn as_integer(&self) -> Result<u64, DatafileError> {
        let value = self.0.as_u64().ok_or_else(|| {
            // Unable to read property as an integer
            Report::new(DatafileError::InvalidType(TYPE_INTEGER.into()))
        })?;

        Ok(value)
    }

    pub(crate) fn as_boolean(&self) -> Result<bool, DatafileError> {
        let value = self.0.as_bool().ok_or_else(|| {
            // Unable to read property as a boolean
            Report::new(DatafileError::InvalidType(TYPE_BOOLEAN.into()))
        })?;

        Ok(value)
    }

    pub(crate) fn as_array(&mut self) -> Result<impl Iterator<Item = Context> + '_, DatafileError> {
        let iterator = self
            .0
            .as_array_mut()
            .ok_or_else(|| {
                // Unable to read property as an array
                Report::new(DatafileError::InvalidType(TYPE_ARRAY.into()))
            })?
            .iter_mut()
            .map(|value| {
                // Extract the array item from the JSON, ...
                let value = value.take();
                // ... and wrap it in a Json struct
                Context(value)
            });

        Ok(iterator)
    }
}

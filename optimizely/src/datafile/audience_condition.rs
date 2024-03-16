use std::cmp::Ordering;

// External imports
use num_ord::NumOrd;
use serde::Deserialize;
use serde_json::value::{Number, Value};

// Imports from crate
use crate::client::UserAttributes;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum AudienceCondition {
    #[serde(rename = "custom_attribute")]
    CustomAttribute(CustomAttributeCondition),
    // #[serde(rename = "third_party_dimension")]
    // ThirdPartyDimension(ThirdPartyDimensionCondition),
}

impl AudienceCondition {
    // Method to evaluate a condition
    pub fn evaluate(&self, user_attributes: &UserAttributes) -> bool {
        match self {
            AudienceCondition::CustomAttribute(condition) => condition.evaluate(user_attributes),
            // AudienceCondition::ThirdPartyDimension(condition) => condition.evaluate(user_attributes),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "match")]
pub enum CustomAttributeCondition {
    #[serde(rename = "exact")]
    Exact(ExactCondition),
    #[serde(rename = "exists")]
    Exists(ExistsCondition),
    #[serde(rename = "gt")]
    GreaterThan(NumericCondition),
    #[serde(rename = "ge")]
    GreaterThanOrEqualTo(NumericCondition),
    #[serde(rename = "lt")]
    LessThan(NumericCondition),
    #[serde(rename = "le")]
    LessThanOrEqualTo(NumericCondition),
    #[serde(rename = "substring")]
    Substring(SubstringCondition),
    // #[serde(rename = "semver_eq")]
    // SemverEqualTo(SemverCondition),
    // #[serde(rename = "semver_gt")]
    // SemverGreaterThan(SemverCondition),
    // #[serde(rename = "semver_ge")]
    // SemverGreaterThanOrEqualTo(SemverCondition),
    // #[serde(rename = "semver_lt")]
    // SemverLessThan(SemverCondition),
    // #[serde(rename = "semver_le")]
    // SemverLessThanOrEqualTo(SemverCondition),
    #[serde(other)]
    Unknown,
}

impl CustomAttributeCondition {
    // Method to evaluate a condition
    pub fn evaluate(&self, user_attributes: &UserAttributes) -> bool {
        match self {
            CustomAttributeCondition::Exact(condition) => condition.evaluate(user_attributes),
            CustomAttributeCondition::Exists(condition) => condition.evaluate(user_attributes),
            CustomAttributeCondition::GreaterThan(condition) => condition
                .compare(user_attributes)
                .is_some_and(|x| x.is_gt()),
            CustomAttributeCondition::GreaterThanOrEqualTo(condition) => condition
                .compare(user_attributes)
                .is_some_and(|x| x.is_ge()),
            CustomAttributeCondition::LessThan(condition) => condition
                .compare(user_attributes)
                .is_some_and(|x| x.is_lt()),
            CustomAttributeCondition::LessThanOrEqualTo(condition) => condition
                .compare(user_attributes)
                .is_some_and(|x| x.is_le()),
            CustomAttributeCondition::Substring(condition) => condition.evaluate(user_attributes),
            CustomAttributeCondition::Unknown => {
                log::warn!("unrecognized match type in audience condition");
                false
            }
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ExactCondition {
    pub name: String,
    pub value: Value,
}

impl ExactCondition {
    // Method to evaluate a condition
    pub fn evaluate(&self, user_attributes: &UserAttributes) -> bool {
        let optional_user_value = user_attributes.get(&self.name);
        if optional_user_value.is_none() {
            return false;
        }
        let user_value = optional_user_value.unwrap();
        if user_value.is_null() {
            return false;
        }
        match &self.value {
            Value::Bool(condition_value) => {
                return user_value.as_bool().is_some_and(|x| x == condition_value);
            }
            Value::Number(condition_value) => {
                return user_value.as_number().is_some_and(|x| x == condition_value);
            }
            Value::String(condition_value) => {
                return user_value.as_str().is_some_and(|x| x == condition_value);
            }
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ExistsCondition {
    name: String,
}

impl ExistsCondition {
    // Method to evaluate a condition
    pub fn evaluate(&self, user_attributes: &UserAttributes) -> bool {
        user_attributes
            .get(&self.name)
            .is_some_and(|attribute| !attribute.is_null())
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct NumericCondition {
    name: String,
    value: Number,
}

impl NumericCondition {
    // Returns Some<Equal> if user_attributes.get(self.name) == self.value
    // Returns Some<Greater> if user_attributes.get(self.name) > self.value
    // Returns Some<Less> if user_attributes.get(self.name) < self.value
    // Returns None if user_attributes.get(self.name) is absent, not a number, or "incomparable"
    // to self.value. I'm not sure what incomparable means, but I think `NaN` is probably
    // incomparable to everything
    pub fn compare(&self, user_attributes: &UserAttributes) -> Option<Ordering> {
        if let Some(crate::client::AttributeValue::Number(attribute_value)) = user_attributes.get(&self.name) {
            if let Some(attribute_u64) = attribute_value.as_u64() {
                if let Some(condition_u64) = self.value.as_u64() {
                    return Some(attribute_u64.cmp(&condition_u64));
                } else if let Some(condition_i64) = self.value.as_i64() {
                    return NumOrd(attribute_u64).partial_cmp(&NumOrd(condition_i64));
                } else if let Some(condition_f64) = self.value.as_f64() {
                    return NumOrd(attribute_u64).partial_cmp(&NumOrd(condition_f64));
                }
            } else if let Some(attribute_i64) = attribute_value.as_i64() {
                if let Some(condition_u64) = self.value.as_u64() {
                    return NumOrd(attribute_i64).partial_cmp(&NumOrd(condition_u64));
                } else if let Some(condition_i64) = self.value.as_i64() {
                    return Some(attribute_i64.cmp(&condition_i64));
                } else if let Some(condition_f64) = self.value.as_f64() {
                    return NumOrd(attribute_i64).partial_cmp(&NumOrd(condition_f64));
                }
            } else if let Some(attribute_f64) = attribute_value.as_f64() {
                if let Some(condition_u64) = self.value.as_u64() {
                    return NumOrd(attribute_f64).partial_cmp(&NumOrd(condition_u64));
                } else if let Some(condition_i64) = self.value.as_i64() {
                    return NumOrd(attribute_f64).partial_cmp(&NumOrd(condition_i64));
                } else if let Some(condition_f64) = self.value.as_f64() {
                    return NumOrd(attribute_f64).partial_cmp(&NumOrd(condition_f64));
                }
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SubstringCondition {
    pub name: String,
    pub value: String,
}

impl SubstringCondition {
    // Method to evaluate a condition
    pub fn evaluate(&self, user_attributes: &UserAttributes) -> bool {
        return user_attributes.get(&self.name).is_some_and(|attribute| {
            attribute
                .as_str()
                .is_some_and(|str_attribute| str_attribute.contains(&self.value))
        });
    }
}

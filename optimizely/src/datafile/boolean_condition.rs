use serde::de::value::{
    BoolDeserializer, BytesDeserializer, F64Deserializer, I128Deserializer, I64Deserializer, MapAccessDeserializer,
    StrDeserializer, U128Deserializer, U64Deserializer,
};
// External imports
use serde::{de::Visitor, Deserialize, Deserializer};
use serde_value::{Value, ValueDeserializer};
use std::cmp;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub enum BooleanCondition<T> {
    And(Vec<Box<BooleanCondition<T>>>),
    Or(Vec<Box<BooleanCondition<T>>>),
    Not(Option<Box<BooleanCondition<T>>>),
    Single(T),
}

impl<'de, T> Deserialize<'de> for BooleanCondition<T>
where
    T: Deserialize<'de>,
{
    // Method to deserialize an array into a BooleanCondition
    fn deserialize<D>(deserializer: D) -> Result<BooleanCondition<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BooleanConditionVisitor<T> {
            // https://serde.rs/deserialize-map.html
            marker: PhantomData<fn() -> BooleanCondition<T>>,
        }

        impl<T> BooleanConditionVisitor<T> {
            fn new() -> Self {
                BooleanConditionVisitor { marker: PhantomData }
            }
        }

        impl<'de, T> Visitor<'de> for BooleanConditionVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = BooleanCondition<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Examples: ["or", 1, 2, 3]; ["or", ["and", { "name": "thing1" }, { "name": "thing2" }, { "name": "thing3" }]]
                formatter.write_str(
                    "an array whose first element is 'and', 'or', or 'not' and subsequent elements are T, or another array",
                )
            }

            // Most of these visit_... functions just say, "deserialize it as a `T`, then wrap it in a BooleanCondition::Single"
            // visit_seq is the only exeception
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanCondition::Single(T::deserialize(BoolDeserializer::new(v))?))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanCondition::Single(T::deserialize(BytesDeserializer::new(v))?))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanCondition::Single(T::deserialize(F64Deserializer::new(v))?))
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanCondition::Single(T::deserialize(I128Deserializer::new(v))?))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanCondition::Single(T::deserialize(I64Deserializer::new(v))?))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanCondition::Single(T::deserialize(StrDeserializer::new(v))?))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanCondition::Single(T::deserialize(U128Deserializer::new(v))?))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanCondition::Single(T::deserialize(U64Deserializer::new(v))?))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                Ok(BooleanCondition::Single(T::deserialize(MapAccessDeserializer::new(map))?))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut condition: BooleanCondition<T>;
                let size_hint = seq.size_hint().unwrap_or(0);
                if let Some(first_element) = seq.next_element::<Value>()? {
                    match first_element {
                        Value::String(ref first_str) => {
                            if first_str == "and" {
                                condition = BooleanCondition::And(Vec::with_capacity(cmp::max(1, size_hint) - 1));
                            } else if first_str == "or" {
                                condition = BooleanCondition::Or(Vec::with_capacity(cmp::max(1, size_hint) - 1));
                            } else if first_str == "not" {
                                return Ok(BooleanCondition::Not(seq.next_element()?));
                            } else {
                                // https://stackoverflow.com/questions/74461366/how-do-i-deserialize-the-last-element-of-a-serde-sequence-differently-from-the-r
                                let mut vec = Vec::with_capacity(size_hint);
                                vec.push(Box::new(BooleanCondition::<T>::deserialize(ValueDeserializer::new(
                                    first_element,
                                ))?));
                                condition = BooleanCondition::Or(vec)
                            }
                        }
                        _ => {
                            let mut vec = Vec::with_capacity(size_hint);
                            vec.push(Box::new(BooleanCondition::<T>::deserialize(ValueDeserializer::new(
                                first_element,
                            ))?));
                            condition = BooleanCondition::Or(vec)
                        }
                    }
                    while let Some(next_element) = seq.next_element::<BooleanCondition<T>>()? {
                        match condition {
                            BooleanCondition::And(ref mut vec) => vec.push(Box::new(next_element)),
                            BooleanCondition::Or(ref mut vec) => vec.push(Box::new(next_element)),
                            _ => (),
                        }
                    }
                    Ok(condition)
                } else {
                    Ok(BooleanCondition::And(Vec::new()))
                }
            }
        }

        deserializer.deserialize_any(BooleanConditionVisitor::new())
    }
}

impl<T> BooleanCondition<T> {
    // Method to evaluate a condition
    pub fn evaluate<E>(&self, evaluator: &E) -> bool
    where
        E: Fn(&T) -> bool,
    {
        match self {
            BooleanCondition::And(conditions) => conditions
                .iter()
                .all(|condition| condition.evaluate(evaluator)),
            BooleanCondition::Or(conditions) => conditions
                .iter()
                .any(|condition| condition.evaluate(evaluator)),
            BooleanCondition::Not(option) => {
                if let Some(condition) = option {
                    return !condition.evaluate(evaluator);
                }
                return false;
            }
            BooleanCondition::Single(condition) => evaluator(&condition),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            BooleanCondition::And(conditions) => conditions.is_empty(),
            BooleanCondition::Or(conditions) => conditions.is_empty(),
            BooleanCondition::Not(conditions) => conditions.is_none(),
            _ => false,
        }
    }
}

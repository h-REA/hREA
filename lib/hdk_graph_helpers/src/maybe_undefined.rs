use serde::{ de::Deserialize, de::Deserializer };
use hdk::{
    holochain_core_types::{
        json::JsonString,
        error::HolochainError,
    },
};

/// Type alias for dealing with entry fields that are not provided separately to nulls.
/// Used for update behaviour- null erases fields, undefined leaves them untouched.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum MaybeUndefined<T> {
    None,
    Some(T),
    Undefined,
}

// helper method for pulling values out to regular Option
// :TODO: see if this can be done without cloning the wrapped data for complex types like `Vec`
impl<T> MaybeUndefined<T> where T: Clone {
    pub fn to_option(self) -> Option<T> {
        match self {
            MaybeUndefined::Some(val) => Option::Some(val.clone()),
            _ => None,
        }
    }
}

impl<T> Into<Option<T>> for MaybeUndefined<T> where T: Clone {
    fn into(self) -> Option<T> {
        self.to_option()
    }
}

impl<T> From<Option<T>> for MaybeUndefined<T> {
    fn from(opt: Option<T>) -> MaybeUndefined<T> {
        match opt {
            Some(v) => MaybeUndefined::Some(v),
            None => MaybeUndefined::None,
        }
    }
}

// default to undefined, not null
// used by Serde to provide default values via `#[serde(default)]`
impl<T> Default for MaybeUndefined<T> {
    fn default() -> MaybeUndefined<T> {
        MaybeUndefined::Undefined
    }
}

// deserialize via standard Option behaviour, then typecast across
impl<'de, T> Deserialize<'de> for MaybeUndefined<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use hdk::holochain_core_types_derive::{ DefaultJson };

    #[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone, PartialEq)]
    struct TestEntry {
        #[serde(default)]
        test_field: MaybeUndefined<Vec<String>>,
    }

    impl TestEntry {
        pub fn getter(&self) -> Option<Vec<String>> {
            self.test_field.clone().to_option()
        }
    }

    #[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone, PartialEq)]
    struct TestEntrySimple {
        #[serde(default)]
        test_field: MaybeUndefined<String>,
    }

    // Ensures that pulling data out of optional fields is possible when the data has move semantics
    #[test]
    fn test_vector_ownership() {
        let entry = TestEntry { test_field: MaybeUndefined::Some(vec!["blah".to_string()]) };
        let copied = entry.getter();
        let another: TestEntry = entry.into();
    }

    #[test]
    fn test_deserialization_some() {
        let expected = TestEntrySimple { test_field: MaybeUndefined::Some("blah".to_string()) };
        let input_json = "{\"test_field\":\"blah\"}";

        assert_eq!(
            Ok(expected),
            TestEntrySimple::try_from(JsonString::from_json(input_json)),
        );
    }

    #[test]
    fn test_deserialization_none() {
        assert_eq!(
            Ok(TestEntrySimple { test_field: MaybeUndefined::None }),
            TestEntrySimple::try_from(JsonString::from_json("{\"test_field\":null}")),
        );

        assert_ne!(
            Ok(TestEntrySimple { test_field: MaybeUndefined::Undefined }),
            TestEntrySimple::try_from(JsonString::from_json("{\"test_field\":null}")),
        );
    }

    #[test]
    fn test_deserialization_undefined() {
        assert_eq!(
            Ok(TestEntrySimple { test_field: MaybeUndefined::Undefined }),
            TestEntrySimple::try_from(JsonString::from_json("{}")),
        );

        assert_ne!(
            Ok(TestEntrySimple { test_field: MaybeUndefined::Undefined }),
            TestEntrySimple::try_from(JsonString::from_json("{\"test_field\":null}")),
        );
    }
}

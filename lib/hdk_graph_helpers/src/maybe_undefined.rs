use serde::{ de::Deserialize, de::Deserializer };
use serde::ser::{Serialize, Serializer};

/// Type alias for dealing with entry fields that are not provided separately to nulls.
/// Used for update behaviour- null erases fields, undefined leaves them untouched.
#[derive(Debug, Clone, PartialEq)]
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
    pub fn unwrap(self) -> T {
        match self {
            MaybeUndefined::Some(val) => val.clone(),
            MaybeUndefined::None => panic!("Attempted to unwrap on a MaybeUndefined::None value"),
            MaybeUndefined::Undefined => panic!("Attempted to unwrap on a MaybeUndefined::Undefined value"),
        }
    }
}

impl<T> MaybeUndefined<T> {
    pub fn is_undefined(&self) -> bool {
        match self {
            MaybeUndefined::Undefined => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            MaybeUndefined::None => true,
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            MaybeUndefined::Some(_) => true,
            _ => false,
        }
    }
}

/// handler usage: #[serde(default = "hdk_graph_helpers::maybe_undefined::default_false")]
pub fn default_false() -> MaybeUndefined<bool> {
    MaybeUndefined::Some(false)
}

/// handler usage: #[serde(default = "hdk_graph_helpers::maybe_undefined::default_true")]
pub fn default_true() -> MaybeUndefined<bool> {
    MaybeUndefined::Some(true)
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

// serialize such that null / undefined is
impl<T> Serialize for MaybeUndefined<T>
    where T: serde::Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        match self {
            MaybeUndefined::Some(val) => serializer.serialize_some(&Some(val)),
            _ => serializer.serialize_none(),
            // :TODO: optimally the type could be of this rather than requiring that fields be set with skip_serializing_if
            // MaybeUndefined::None => serializer.serialize_none(),
            // MaybeUndefined::Undefined => serializer.serialize_unit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use hdk::holochain_json_api::{
        json::JsonString,
        error::JsonError,
    };
    use holochain_json_derive::{ DefaultJson };

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
        let _copied = entry.getter();
        let _another: TestEntry = entry.into();
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

    #[test]
    fn test_serialization_some() {
        assert_eq!(
            "{\"test_field\":\"blah\"}".to_string(),
            serde_json::to_string(&TestEntrySimple { test_field: MaybeUndefined::Some("blah".to_string()) }).unwrap(),
        );
    }

    #[test]
    fn test_serialization_none() {
        assert_eq!(
            "{\"test_field\":null}".to_string(),
            serde_json::to_string(&TestEntrySimple { test_field: MaybeUndefined::None }).unwrap(),
        );
        // :TODO:
        // assert_ne!(
        //     "{\"test_field\":null}".to_string(),
        //     serde_json::to_string(&TestEntrySimple { test_field: MaybeUndefined::Undefined }).unwrap(),
        // );
    }

    #[test]
    fn test_serialization_undefined() {
        // :TODO:
        // assert_eq!(
        //     "{}".to_string(),
        //     serde_json::to_string(&TestEntrySimple { test_field: MaybeUndefined::Undefined }).unwrap(),
        // );
        assert_ne!(
            "{\"test_field\":null}".to_string(),
            serde_json::to_string(&TestEntrySimple { test_field: MaybeUndefined::Undefined }).unwrap(),
        );
    }
}

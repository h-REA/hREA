use std::convert::TryFrom;
use hdk::{
    holochain_core_types::{
        json::JsonString,
        error::HolochainError,
    },
};

/// Type alias for dealing with entry fields that are not provided separately to nulls.
/// Used for update behaviour- null erases fields, undefined leaves them untouched.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

// default to undefined, not null
// used by Serde to provide default values via `#[serde(default)]`
impl<T> Default for MaybeUndefined<T> {
    fn default() -> MaybeUndefined<T> {
        MaybeUndefined::Undefined
    }
}

// impl<'a, T> TryFrom<&'a JsonString> for MaybeUndefined<T> {
//     type Error = HolochainError;
//     fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
//         match ::serde_json::from_str(&String::from(json_string)) {
//             Ok(d) => Ok(d),
//             Err(e) => Err(HolochainError::SerializationError(e.to_string())),
//         }
//     }
// }

// impl<T> TryFrom<JsonString> for MaybeUndefined<T> {
//     type Error = HolochainError;
//     fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
//         MaybeUndefined<T>::try_from(&json_string)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use hdk::holochain_core_types_derive::{ DefaultJson };

    #[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
    struct TestEntry {
        #[serde(default)]
        test_field: MaybeUndefined<Vec<String>>,
    }

    impl TestEntry {
        pub fn getter(&self) -> Option<Vec<String>> {
            self.test_field.clone().to_option()
        }
    }

    // Ensures that pulling data out of optional fields is possible when the data has move semantics
    #[test]
    fn test_vector_ownership() {
        let entry = TestEntry { test_field: MaybeUndefined::Some(vec!["blah".to_string()]) };
        let copied = entry.getter();
        let another: TestEntry = entry.into();
    }
}

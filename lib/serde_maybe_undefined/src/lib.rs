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
            MaybeUndefined::Some(val) => Option::Some(val),
            _ => None,
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            MaybeUndefined::Some(val) => val,
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

    pub fn is_none_or_undefined(&self) -> bool {
        match self {
            MaybeUndefined::Undefined => true,
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

/// handler usage: #[serde(default = "serde_maybe_undefined::default_false")]
pub fn default_false() -> MaybeUndefined<bool> {
    MaybeUndefined::Some(false)
}

/// handler usage: #[serde(default = "serde_maybe_undefined::default_true")]
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
            // MaybeUndefined::Undefined => serializer.serialize_bytes("".as_bytes()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hdk::prelude::*;

    #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
    struct TestEntry {
        #[serde(default)]
        #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
        test_field: MaybeUndefined<Vec<String>>,
    }

    impl TestEntry {
        pub fn getter(&self) -> Option<Vec<String>> {
            self.test_field.clone().to_option()
        }
    }

    #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
    struct TestEntrySimple {
        #[serde(default)]
        #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
        test_field: MaybeUndefined<String>,
    }

    macro_rules! do_test {
        ( $t:ty, $i:expr ) => {{
            let i = $i;
            let sb: SerializedBytes = i.clone().try_into().unwrap();
            // this isn't for testing it just shows how the debug output looks
            println!("{:?}", &sb);

            let returned: $t = sb.try_into().unwrap();

            assert_eq!(returned, i);
        }};
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

        do_test!(TestEntrySimple, expected);
    }

    #[test]
    fn test_deserialization_none() {
        do_test!(TestEntrySimple, TestEntrySimple { test_field: MaybeUndefined::None });
    }

    #[test]
    fn test_deserialization_undefined() {
        do_test!(TestEntrySimple, TestEntrySimple { test_field: MaybeUndefined::Undefined });
    }
}

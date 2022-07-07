/**
 * The primary goal of this module is to provide structs which ensure universal uniqueness
 * of record identifiers in Holochain apps. This is achieved by concatenating the `DnaHash`
 * of the host network space with an identifier which is locally-unique within that membrane.
 *
 * Such information is sufficient to build a universally-unique Holochain URI, and allows
 * apps to mix references to disparate network spaces in the same data structures.
 *
 * To convert wrapped values to an `EntryHash` or `DnaHash`, use `aliased_val.as_ref()` in assignment
 * to the appropriate type.
 *
 * A secondary goal is to provide an ability to create distinct types for different identifiers,
 * such that identifiers cannot be accidentally mismatched to the wrong record types.
 * For example, given these two definitions-
 *
 *  addressable_identifier!(CommitmentAddress => EntryHash);
 *  addressable_identifier!(IntentAddress => EntryHash);
 *
 * 'CommitmentAddress' and 'IntentAddress' cannot be confused even though they contain data
 * of the same format, and the compiler will complain if a 'CommitmentAddress' is incorrectly
 * assigned to a struct field or method parameter expecting an 'IntentAddress'. This helps to
 * prevent developer error when your application has a large number of different entry types.
 *
 * This same functionality is also provided for simple values with the `simple_alias` macro.
 */
use std::fmt::Debug;
pub use hdk::prelude::*;
pub use hdk;
pub use holo_hash::*;

/// Generate a simple newtype wrapper around some raw data, to enforce distinctness of
/// different data items with the same underlying format.
///
#[macro_export]
macro_rules! simple_alias {
    ($id:ident => $base:ty) => {
        #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
        pub struct $id(pub $base);

        impl From<$id> for $base {
            fn from(v: $id) -> $base {
                v.0
            }
        }

        impl From<$base> for $id {
            fn from (v: $base) -> $id {
                $id(v)
            }
        }

        impl AsRef<$base> for $id {
            fn as_ref(&self) -> &$base {
                &self.0
            }
        }
    }
}

/// Supertrait to bind all dependent traits that implement unique identifier behaviours.
///
pub trait DnaAddressable<B>
    where Self: Clone + Eq + std::hash::Hash
            + Debug + Into<Vec<u8>> + serde::Serialize
            + AsRef<DnaHash> + AsRef<B>,
        B: Clone,
        AnyDhtHash: From<B>,
{
    fn new(dna: DnaHash, identifier: B) -> Self;
}

/// Generate a universally-unique identifier for some DNA-local identifier
/// (an `EntryHash` or `AgentPubKey`).
///
/// This also defines an `EntryDef` of the same name so that the identifier
/// can be directly stored to the DHT, which is required for building foreign-key
/// indexes which reference remote data.
///
#[macro_export]
macro_rules! addressable_identifier {
    ($r:ident => $base:ty) => {
        // externally facing type, with DnaHash of cell for universally-unique context
        #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $r(pub DnaHash, pub $base);

        // define as an EntryDef so identifiers can be stored directly to the DHT as indexing anchors
        entry_def!($r EntryDef {
            id: stringify!($r).into(),
            required_validations: RequiredValidations::default(),
            visibility: EntryVisibility::Public,
            required_validation_type: RequiredValidationType::default(),
        });

        // constructor
        impl $crate::DnaAddressable<$base> for $r {
            fn new(dna: DnaHash, identifier: $base) -> Self {
                Self(dna, identifier)
            }
        }

        // reference raw cell-local identifier from externally facing type
        impl AsRef<$base> for $r {
            fn as_ref(&self) -> &$base {
                &self.1
            }
        }

        // reference DnaHash from externally facing type
        impl AsRef<DnaHash> for $r {
            fn as_ref(&self) -> &DnaHash {
                &self.0
            }
        }

        // convert to raw bytes in serializing
        impl Into<Vec<u8>> for $r {
            fn into(self) -> Vec<u8> {
                $crate::extern_id_to_bytes::<Self, $base>(&self)
            }
        }
    }
}

/// Supertrait for things which can be identified by some string label in a particular DNA
///
pub trait DnaIdentifiable<B>
    where Self: Clone + Debug + Eq + std::hash::Hash + AsRef<DnaHash> + AsRef<B>,
        B: Clone + AsRef<str>,
{
    fn new(dna: DnaHash, identifier: B) -> Self;
}

/// Generate a universally-unique identifier for some DNA-local string identifier.
/// The implementor must ensure that this string ID remains unique in the DNA via
/// whatever application logic is relevant to the use-case.
///
#[macro_export]
macro_rules! dna_scoped_string {
    ($r:ident) => {
        // externally facing type, with DnaHash of cell for context
        #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $r(pub DnaHash, pub String);

        // constructor
        impl $crate::DnaIdentifiable<String> for $r {
            fn new(dna: DnaHash, identifier: String) -> Self {
                Self(dna, identifier)
            }
        }

        // reference raw cell-local identifier from externally facing type
        impl AsRef<String> for $r {
            fn as_ref(&self) -> &String {
                &self.1
            }
        }

        // reference DnaHash from externally facing type
        impl AsRef<DnaHash> for $r {
            fn as_ref(&self) -> &DnaHash {
                &self.0
            }
        }
    }
}

/// Convert an externally-facing identifier (`AnyDhtHash` + `DnaHash`) into raw bytes for serializing
/// in an I/O payload or `Path` `Component`.
///
/// Use the `addressable_identifier!` macro to auto-implement type-specific identifiers compatible with this method of encoding.
///
/// :TODO: remove this method, it's currently used in conversion of IDs to cursors in response formatting and
/// should probably be replaced with the HoloHashB64 variants or similar functionality.
///
pub fn extern_id_to_bytes<A, B>(id: &A) -> Vec<u8>
    where A: AsRef<DnaHash> + AsRef<B>,
        B: Clone,
        AnyDhtHash: From<B>,
{
    // use single identifier to combine AnyDhtHash + DnaHash
    // place AnyDhtHash before DnaHash to aid in sharding strategies that look at header bytes
    let entry_address: &B = id.as_ref();
    let dna_hash: &DnaHash = id.as_ref();

    [AnyDhtHash::from((*entry_address).clone()).get_raw_36(), dna_hash.get_raw_36()].concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use holo_hash::HOLO_HASH_UNTYPED_LEN;

    #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
    pub struct SomeValue(pub String);

    #[test]
    fn test_addressable_type() {
        addressable_identifier!(Ident => SomeValue);

        let base = SomeValue("test".to_string());
        let wrapped: Ident = base.clone().into();
        let external: IdentRemote = (DnaHash::from_raw_36(vec![0xdb; HOLO_HASH_UNTYPED_LEN]), wrapped).into();
        let extracted: SomeValue = external.into();

        assert_eq!(base, extracted, "Original data matches wrapped, externalised, extracted roundtrip data");
    }
}

/**
 * Type aliases used to ensure explicit awareness of applicable record types in VF structs
 *
 * To convert wrapped values to an `EntryHash`, use `aliased_val.as_ref()`.
 * To convert a plain `EntryHash` to its wrapped form, use `raw_address.into()`.
 */
use std::fmt::Debug;

pub use holochain_serialized_bytes::prelude::*;
pub use holo_hash::{DnaHash, EntryHash, HeaderHash, AnyDhtHash, HOLO_HASH_UNTYPED_LEN};

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

/// Supertrait to bind all dependent traits that implement identifier behaviours.
///
pub trait DnaAddressable<B>
    where Self: Clone + Debug + Eq + std::hash::Hash + AsRef<DnaHash> + AsRef<B> + Into<Vec<u8>>,
        B: Clone,
        AnyDhtHash: From<B>,
{
    fn new(dna: DnaHash, identifier: B) -> Self;
}

#[macro_export]
macro_rules! addressable_identifier {
    ($r:ident => $base:ty) => {
        // externally facing type, with DnaHash of cell for context
        #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $r(pub DnaHash, pub $base);

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
                extern_id_to_bytes::<Self, $base>(&self)
            }
        }
    }
}

addressable_identifier!(RevisionHash => HeaderHash);

/// Supertrait for things which can be identified by some string label in a particular DNA
///
pub trait DnaIdentifiable<B>
    where Self: Clone + Debug + Eq + std::hash::Hash + AsRef<DnaHash> + AsRef<B>,
        B: Clone + AsRef<str>,
{
    fn new(dna: DnaHash, identifier: B) -> Self;
}

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

/// Convert raw bytes encoded into a `Path` index into its full identity pair
///
/// @see hdk_type_serialization_macros::extern_id_to_bytes
///
pub fn bytes_to_extern_id<A>(key_bytes: &[u8]) -> Result<A, SerializedBytesError>
    where A: DnaAddressable<EntryHash>,
{
    if key_bytes.len() != HOLO_HASH_UNTYPED_LEN * 2 { return Err(SerializedBytesError::Deserialize("Invalid input length for bytes_to_extern_id!".to_string())) }

    // pull DnaHash from last 36 bytes; first 36 are for EntryHash/HeaderHash
    // @see holo_hash::hash
    Ok(A::new(
        DnaHash::from_raw_36(key_bytes[HOLO_HASH_UNTYPED_LEN..].to_vec()),
        EntryHash::from_raw_36(key_bytes[0..HOLO_HASH_UNTYPED_LEN].to_vec()),
    ))
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

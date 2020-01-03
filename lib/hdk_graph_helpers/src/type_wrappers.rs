/**
 * Newtype wrappers for Holochain core types
 *
 * Used as stand-ins where generic type parameters are necessary, since use of newtypes
 * makes the underlying type objects incompatible.
 *
 * @see vf_core::type_aliases
*/
use hdk::{
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
};
use holochain_json_derive::{ DefaultJson };

// :DUPE: newtype-macro-rules
macro_rules! simple_alias {
    ($id:ident => $base:ty) => {
        #[derive(Serialize, Deserialize, DefaultJson, Debug, Clone, PartialEq)]
        pub struct $id($base);

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
    };
}

// :SHONK: a generic Address type wrapper for use in situations where `Address` would normally be used.
// Provides a compatibility layer for methods which accept `AsRef<Address>`.
simple_alias!(Addressable => Address);

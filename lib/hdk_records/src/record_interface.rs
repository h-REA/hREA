/**
 * Record-handling interface for graph-like record structures on Holochain.
 *
 * Record implementations should extend from these traits in order to implement an API
 * compatible with the publicly exposed helpers in this crate.
 *
 * @package HoloREA
 * @since   2019-07-02
 */

use hdk::prelude::*;
use hdk_uuid_types::DnaAddressable;

use crate::{
    RecordAPIResult,
};

/// A trait for managing records associated with a consistent "base" identifier.
///
/// To be implemented by the wrapper type which adds uniquely identifying data
/// to another data structure `T`.
///
/// @see generate_record_entry!
///

pub trait Identified<T, A>
    where Entry: TryFrom<Self>,
        Self: Sized,
        A: DnaAddressable<EntryHash>,
{
    fn entry(&self) -> T;
    fn identity(&self) -> RecordAPIResult<A>;
}

/// A trait for managing records associated with a consistent "base" identifier.
///
/// To be implemented by the actual entry data type in order to bind non-identified
/// data to its appropriate `Identified` wrapper.
///
/// @see generate_record_entry!
///
pub trait Identifiable<T> {
    fn with_identity(&self, id_hash: Option<EntryHash>) -> T;
}

/// Compose an `Identified` structure around the provided entry struct, in order to provide
/// consistent identities to linked entry information which models updates to some data over time.
///
/// In addition, the original entry struct receives an `Identifiable` trait impl that can be used
/// to generate the storage data struct by assigning the previously known unique entry identifier.
///
#[macro_export]
macro_rules! generate_record_entry {
    ( $( $t:ident, $id:ident, $to:ident );+ ) => {
        $(
            // $crate::paste::paste! {

                #[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
                pub struct $to {
                    entry: $t,
                    id_hash: Option<$crate::EntryHash>, // :NOTE: None for first record
                }

                app_entry!($to);

                #[hdk_entry_defs]
                #[unit_enum(UnitEntryType)]
                pub enum EntryTypes {
                    $to($to),
                }

                impl $crate::record_interface::Identified<$t, $id> for $to
                {
                    fn entry(&self) -> $t {
                        self.entry.to_owned()
                    }

                    fn identity(&self) -> $crate::RecordAPIResult<$id> {
                        let dna_hash = dna_info()?.hash;

                        match &self.id_hash {
                            // If there is an ID hash, it points to the identity anchor `Path`
                            Some(identity) => Ok($id(dna_hash, identity.to_owned())),
                            // If no ID hash exists, this is the first entry (@see `create_record()`)
                            None => {
                                let hash = $crate::hash_entry((*self).to_owned())?;
                                Ok($id(dna_hash, hash))
                            },
                        }
                    }
                }

                impl $crate::record_interface::Identifiable<$to> for $t
                {
                    fn with_identity(&self, id_hash: Option<$crate::EntryHash>) -> $to
                    {
                        $to {
                            entry: self.to_owned(),
                            id_hash,
                        }
                    }
                }

                impl From<$to> for EntryTypes
                {
                    fn from(e: $to) -> EntryTypes
                    {
                        EntryTypes::$to(e)
                    }
                }

            // }
        )*
    };
}

/// Interface for Holochain entry structs that can be updated via some predefined logic.
///
/// Defines a structured mechanism for generating new entries from previous entry data
/// and a set of inputs (`T`) that can be used to update the entry.
///
pub trait Updateable<T> {
    /// Updates a Holochain Entry struct (`self`) by processing an update payload
    /// of the bound type `T` against it and returning a new Entry struct holding
    /// the updated data.
    ///
    /// @see hdk_records::record_helpers::update_record
    ///
    fn update_with(&self, e: T) -> Self;
}

/// Interface for obtaining identity information from any data type.
/// Most commonly used for "anchored records" which are retrieved from
/// unique well-known "anchor" entries.
///
/// @see hdk_records::anchored_record_helpers::create_anchored_record
///
pub trait UniquelyIdentifiable {
    fn get_anchor_key(&self) -> RecordAPIResult<String>;
}

/// Provides determination of whether a UniquelyIdentifiable object's
/// identifying information is being modified. Used to trigger anchor
/// index re-updating logic.
///
/// @see hdk_records::anchored_record_helpers::update_anchored_record
///
pub trait UpdateableIdentifier {
    fn get_new_anchor_key(&self) -> Option<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
    pub struct TestEntry {
        field: Option<String>,
    }
    generate_record_entry!(TestEntry, TestEntryWithIdentity);

    #[test]
    fn test_identified_trait() {
        let entry = TestEntry { field: None };
        assert_eq!(
            entry.with_identity(None),
            TestEntryWithIdentity {
                entry: TestEntry { field: None },
                id_hash: None,
            }
        );
        assert_eq!(
            entry.with_identity(None).entry(),
            entry,
        );
    }
}

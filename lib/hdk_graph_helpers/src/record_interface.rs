/**
 * Record-handling interface for graph-like record structures on Holochain.
 *
 * Record implementations should extend from these traits in order to implement an API
 * compatible with the publicly exposed helpers in this crate.
 *
 * @package HoloREA
 * @since   2019-07-02
 */

use hdk3::prelude::*;

use crate::{
    GraphAPIResult,
};

/// A trait for managing records associated with a consistent "base" identifier.
///
/// To be implemented by the wrapper type which adds uniquely identifying data
/// to another data structure `T`.
///
/// @see bind_identity!
///

pub trait Identified<T> {
    fn entry(&self) -> &T;
    fn identity(&self) -> GraphAPIResult<&EntryHash>;
}

/// A trait for managing records associated with a consistent "base" identifier.
///
/// To be implemented by the actual entry data type in order to bind non-identified
/// data to its appropriate `Identified` wrapper.
///
/// @see bind_identity!
///
pub trait Identifiable {
    type StorageType;
    fn with_identity<'a>(&self, identity_entry_hash: Option<EntryHash>) -> Self::StorageType;
}

/// Compose an `Identified` structure around the provided entry struct, in order to provide
/// consistent identities to linked entry information which models updates to some data over time.
///
/// The generated struct assumes the name of the original struct, with a `WithIdentity`
/// suffix. For example, `bind_identity!(ObservedEvent);` will generate an
/// `ObservedEventWithIdentity` struct implementing `Identified<ObservedEvent>`, which
/// should be used to wrap all record data immediately prior to storage.
///
/// In addition, the original `ObservedEvent` receives a `with_identity()` method that can be used
/// to generate the storage data struct by assigning the previously known unique entry identifier.
///
#[macro_export]
macro_rules! bind_identity {
    ( $( $t:ty ),+ ) => {
        $(
            crate::paste::paste! {

                #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
                pub struct [< $t WithIdentity >] {
                    entry: $t,
                    identity_entry_hash: Option<EntryHash>, // :NOTE: None for first record
                }

                impl Identified<$t> for [< $t WithIdentity >]
                {
                    fn entry(&self) -> &$t {
                        &self.entry
                    }

                    fn identity(&self) -> GraphAPIResult<&EntryHash> {
                        match self.identity_entry_hash {
                            // If there is an ID hash, it points to the identity anchor `Path`
                            Some(identity) => Ok(&identity),
                            // If no ID hash exists, this is the first entry (@see `create_record()`)
                            None => {
                                let hash = hash_entry(self.entry())?;
                                Ok(&hash)
                            },
                        }
                    }
                }

                impl Identifiable for $t
                {
                    type StorageType = [< $t WithIdentity >];

                    fn with_identity<'a>(&self, identity_entry_hash: Option<EntryHash>) -> Self::StorageType
                    {
                        [< $t WithIdentity >] {
                            entry: *self,
                            identity_entry_hash: identity_entry_hash.map(|id| { id.into() }),
                        }
                    }

                }

            }
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
    /// @see hdk_graph_helpers::record_helpers::update_record
    ///
    fn update_with(&self, e: &T) -> Self;
}

/// Interface for obtaining identity information from any data type.
/// Most commonly used for "anchored records" which are retrieved from
/// unique well-known "anchor" entries.
///
/// @see hdk_graph_helpers::record_helpers::create_anchored_record
///
pub trait UniquelyIdentifiable {
    fn get_anchor_key(&self) -> Path;
}

/// Provides determination of whether a UniquelyIdentifiable object's
/// identifying information is being modified. Used to trigger anchor
/// index re-updating logic.
///
/// @see hdk_graph_helpers::record_helpers::update_anchored_record
///
pub trait UpdateableIdentifier: UniquelyIdentifiable {
    fn get_new_anchor_key(&self) -> Option<Path>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
    pub struct TestEntry {
        field: Option<String>,
    }
    bind_identity!(TestEntry);

    #[test]
    fn test_identified_trait() {
        let entry = TestEntry { field: None };
        assert_eq!(
            entry.with_identity(None),
            TestEntryWithIdentity {
                entry: TestEntry { field: None },
                identity_entry_hash: None,
            }
        );
        assert_eq!(
            *(entry.with_identity(None).entry()),
            entry,
        );
    }
}

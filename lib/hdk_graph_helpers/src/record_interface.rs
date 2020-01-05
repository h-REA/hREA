/**
 * Record-handling interface for graph-like record structures on Holochain.
 *
 * Record implementations should extend from these traits in order to implement an API
 * compatible with the publicly exposed helpers in this crate.
 *
 * @package HoloREA
 * @since   2019-07-02
 */

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
    fn get_anchor_key(&self) -> String;
}

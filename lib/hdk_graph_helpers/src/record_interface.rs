/**
 * Record-handling interface for graph-like record structures on Holochain.
 *
 * Record implementations should extend from these traits in order to implement an API
 * compliant with the publicly exposed helpers in `record_helpers.rs`.
 *
 * @package HoloREA
 * @since   2019-07-02
 */

pub trait Updateable<T> {
    fn update_with(&self, e: &T) -> Self;
}

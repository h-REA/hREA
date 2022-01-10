use hdk::prelude::*;

/**
 * A `PageInfo` data structure compatible with Relay's connections API that
 * can be included in response payloads for Holochain apps built on the Rust HDK.
 *
 * Contains some additional nonstandard fields that can be useful in client
 * applications if backends are able to determine them.
 *
 * @package hdk_relay_pagination
 * @since   2022-01-10
 */
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    // Cursor pointing to the first of the results returned, to be used with `before` query parameter if the backend supports reverse pagination.
    pub start_cursor: String,
    // Cursor pointing to the last of the results returned, to be used with `after` query parameter if the backend supports forward pagination.
    pub end_cursor: String,
    // True if there are more results before `startCursor`. If unable to be determined, implementations should return `true` to allow for requerying.
    pub has_previous_page: bool,
    // True if there are more results after `endCursor`. If unable to be determined, implementations should return `true` to allow for requerying.
    pub has_next_page: bool,
    // The total result count, if it can be determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<usize>,
    // The number of items requested per page. Allows the storage backend to indicate this when it is responsible for setting a default and the client does not provide it. Note this may be different to the number of items returned, if there is less than 1 page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<usize>,
}

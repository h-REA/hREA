#[cfg(test)]
mod data_structures {
    use ::fixt::prelude::*;
    use chrono::{DateTime};
    use hdk::prelude::{
        set_hdk, MockHdkT,
        Entry,
        Path, PathEntry,
        // GetInput, GetOptions,
        // CreateInput,
        HashInput, HashOutput,
    };
    use holo_hash::{
        fixt::EntryHashFixturator,
        // AnyDhtHash,
    };

    use crate::{ mock_path };

    #[test]
    fn check_written_dht_structures() {
        let mut mock_hdk = MockHdkT::new();
        let index_name = "test_index_name".to_string();

        // mocks for creating the index
        let _index_entry_hash = mock_path!(mock_hdk, index_name.clone());
        let target_entry_hash = mock_path!(mock_hdk, "targeted_entry");

        // set active HDK for framework methods to use
        set_hdk(mock_hdk);

        // write time tree for first record into the index
        hdk_time_indexing::writing::index_entry(
            &index_name,
            target_entry_hash,
            DateTime::parse_from_rfc3339("2020-04-13T03:36:57+00:00").unwrap().into(),
        ).unwrap();

        // :TODO: assert that reading the index DHT structures checks out
    }
}

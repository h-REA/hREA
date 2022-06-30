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

    #[test]
    fn check_written_dht_structures() {
        let mut mock_hdk = MockHdkT::new();
        let index_name = "test_index_name".to_string();
        let index_path = Path::from(index_name.clone());

        // create a target for linking
        let target_entry = Path::from("targeted_entry");

        // mocks for creating the index
        let index_hash = fixt!(EntryHash);
        let index_hash_2 = index_hash.clone();
        mock_hdk
            .expect_hash()
            .with(mockall::predicate::eq(HashInput::Entry(
                Entry::try_from(index_path).unwrap(),
            )))
            .times(1)
            .return_once(|_hash_input| Ok(HashOutput::Entry(index_hash_2)));

        let index_entry = PathEntry::new(index_hash);
        let index_entry_hash = fixt!(EntryHash);
        let index_entry_hash_2 = index_entry_hash.clone();
        mock_hdk
            .expect_hash()
            .with(mockall::predicate::eq(HashInput::Entry(
                Entry::try_from(index_entry).unwrap(),
            )))
            .times(1)
            .return_once(|_hash_input| Ok(HashOutput::Entry(index_entry_hash_2)));

        // mocks for creating the index target entry (in this case, a Path)
        let path_hash = fixt!(EntryHash);
        let path_hash_2 = path_hash.clone();
        mock_hdk
            .expect_hash()
            .with(mockall::predicate::eq(HashInput::Entry(
                Entry::try_from(target_entry).unwrap(),
            )))
            .times(1)
            .return_once(|_hash_input| Ok(HashOutput::Entry(path_hash_2)));

        let path_entry = PathEntry::new(path_hash);
        let path_entry_hash = fixt!(EntryHash);
        let path_entry_hash_2 = path_entry_hash.clone();
        mock_hdk
            .expect_hash()
            .with(mockall::predicate::eq(HashInput::Entry(
                Entry::try_from(path_entry).unwrap(),
            )))
            .times(1)
            .return_once(|_hash_input| Ok(HashOutput::Entry(path_entry_hash_2)));

        // set active HDK for framework methods to use
        set_hdk(mock_hdk);

        // write time tree for first record into the index
        hdk_time_indexing::writing::index_entry(
            &index_name,
            path_entry_hash,
            DateTime::parse_from_rfc3339("2020-04-13T03:36:57+00:00").unwrap().into(),
        ).unwrap();

        // :TODO: assert that reading the index DHT structures checks out
    }
}

#[cfg(test)]
mod tests {
    use ::fixt::prelude::*;
    use hdk::prelude::{
        set_hdk, Entry, GetInput, GetOptions, HashInput, HashOutput, MockHdkT, Path, PathEntry,
    };
    use holo_hash::{fixt::EntryHashFixturator, AnyDhtHash};

    // this test demonstrates that if the index is 'untouched' meaning
    // it can't possibly have children then it will just early exit and
    // return an empty set of results
    #[test]
    fn get_latest_entry_hashes_empty_scenario() {
        let mut mock_hdk = MockHdkT::new();

        let index_name = "test_index_name".to_string();
        let root = Path::from(&index_name);

        // mock the first call to `hash` via `hash_entry` in `Path.path_entry()`
        let path_hash = fixt!(EntryHash);
        let path_hash_2 = path_hash.clone();
        mock_hdk
            .expect_hash()
            .with(mockall::predicate::eq(HashInput::Entry(
                Entry::try_from(root).unwrap(),
            )))
            .times(1)
            .return_once(|_hash_input| Ok(HashOutput::Entry(path_hash)));

        // mock the second call to `hash` via `hash_entry` in `Path.path_entry_hash()`
        let path_entry = PathEntry::new(path_hash_2);
        let path_entry_hash = fixt!(EntryHash);
        let path_entry_hash_2 = path_entry_hash.clone();
        mock_hdk
            .expect_hash()
            .with(mockall::predicate::eq(HashInput::Entry(
                Entry::try_from(path_entry).unwrap(),
            )))
            .times(1)
            .return_once(|_hash_input| Ok(HashOutput::Entry(path_entry_hash)));

        // mock the call to `get` in `Path.exists()`
        mock_hdk
            .expect_get()
            .with(mockall::predicate::eq(vec![GetInput::new(
                AnyDhtHash::from(path_entry_hash_2),
                GetOptions::content(),
            )]))
            .times(1)
            .return_once(|_hash_input| Ok(vec![None]));

        set_hdk(mock_hdk);

        let result = hdk_time_indexing::reading::get_latest_entry_hashes(&index_name, 2);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), vec![]);
    }
}

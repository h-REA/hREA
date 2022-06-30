/// Mock an EntryHash for some entry data, and return it
#[macro_export]
macro_rules! mock_hash {
    (
        $mock_hdk:ident,
        $entry_val:expr
    ) => {
        {
            let entry_hash = fixt!(EntryHash);
            let entry_hash_2 = entry_hash.clone();
            $mock_hdk
                .expect_hash()
                .with(mockall::predicate::eq(HashInput::Entry(
                    Entry::try_from($entry_val).unwrap(),
                )))
                .returning(move |_hash_input| Ok(HashOutput::Entry(entry_hash_2.clone())));

            entry_hash
        }
    };
}

/// Mock a Path, and return the mocked EntryHash for the associated PathEntry
/// (which is what would be stored on the DHT)
#[macro_export]
macro_rules! mock_path {
    (
        $mock_hdk:ident,
        $path_str:expr
    ) => {
        {
            let path_hash = mock_hash!($mock_hdk, Path::from($path_str));
            mock_hash!($mock_hdk, PathEntry::new(path_hash))
        }
    };
}

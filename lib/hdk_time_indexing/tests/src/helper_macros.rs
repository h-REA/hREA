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

/// Mock a read request to the DHT, providing both input and output values
#[macro_export]
macro_rules! mock_get {
    (
        $mock_hdk:ident,
        $input_val:expr,
        $output_val:expr
    ) => {
        $mock_hdk
            .expect_get()
            .with(mockall::predicate::eq(
                vec![GetInput::new(AnyDhtHash::from($input_val), GetOptions::content())]
            ))
            .returning(move |_output| {
                    let signed_action = SignedAction(fixt!(Action), fixt!(Signature));
                    let hashed: HoloHashed<SignedAction> = HoloHashed::from_content_sync(signed_action);
                    let HoloHashed {
                        content: SignedAction(action, signature),
                        hash,
                    } = hashed.clone();

                    Ok(vec![
                        Some(Element::new(
                            SignedActionHashed {
                                hashed: ActionHashed::with_pre_hashed(action, hash),
                                signature,
                            },
                            Some(Entry::try_from($output_val.clone()).unwrap()),
                        ))
                    ])
                });
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
            let path_entry = PathEntry::new(path_hash);
            let path_entry_2 = path_entry.clone();
            let path_entry_hash = mock_hash!($mock_hdk, path_entry);
            let path_entry_hash_2 = path_entry_hash.clone();

            mock_get!($mock_hdk, path_entry_hash_2, path_entry_2);

            path_entry_hash
        }
    };
}

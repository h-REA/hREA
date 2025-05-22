use hdk::prelude::*;
use hrea_integrity::*;
use serde::{Deserialize, Serialize};
use serde_json;

/// Merges fields from `latest` into `current`, keeping `current`'s values if they are `Some`.
pub fn merge_fields<T>(current: T, latest: T) -> T
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let mut current_map: serde_json::Map<String, serde_json::Value> =
        serde_json::to_value(current).unwrap().as_object().unwrap().clone();
    let latest_map: serde_json::Map<String, serde_json::Value> =
        serde_json::to_value(latest).unwrap().as_object().unwrap().clone();

    for (key, value) in latest_map {
        if !current_map.contains_key(&key) || current_map[&key].is_null() {
            current_map.insert(key, value);
        }
    }

    serde_json::from_value(serde_json::Value::Object(current_map)).unwrap()
}

pub fn update_link(
    new_from_hash: AnyLinkableHash,
    new_to_hash: ActionHash,
    link_name: LinkTypes,
    original_to_hash: AnyLinkableHash,
) -> ExternResult<()> {
    debug!("Updating link from {} to {}", original_to_hash, new_to_hash);
    // let path = Path::from(path_str).path_entry_hash()?;
    delete_links(
        new_from_hash.clone(),
        original_to_hash.clone(),
        link_name.clone(),
    )?;
    let tag_prefix = LinkTag(original_to_hash.get_raw_39().to_vec()); // Convert ActionHash to Vec<u8>
    create_link(
        new_from_hash,
        new_to_hash,
        link_name,
        tag_prefix,
    )?;
    Ok(())
}
    
pub fn delete_links(
    from_hash: AnyLinkableHash,
    original_to_hash: AnyLinkableHash,
    link_name: LinkTypes,
) -> ExternResult<()> {
    let tag_prefix = LinkTag(original_to_hash.get_raw_39().to_vec()); // Convert ActionHash to Vec<u8>

    let links = get_links(
        GetLinksInput {
            base_address: from_hash.into(),
            link_type: link_name.try_into_filter()?,
            get_options: GetOptions::default(),
            tag_prefix: Some(tag_prefix),
            before: None,
            after: None,
            author: None,
        }
    )?;
    debug!("Links to delete: {:?}", links);
    for link in links {
        delete_link(link.create_link_hash)?;
    }
    Ok(())
}

pub (crate) fn try_decode_entry<T>(entry: RecordEntry) -> Result<T, SerializedBytesError>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    match entry {
        RecordEntry::Present(entry) => {
            let app_entry_bytes = entry.as_app_entry().expect("Expected AppEntryBytes but found None");
            let decoded: T = app_entry_bytes.clone().into_sb().try_into()?;
            Ok(decoded)
        },
        _ => Err(
            SerializedBytesError::Deserialize("Expected AppEntryBytes but found None".into())
        )
    }
}
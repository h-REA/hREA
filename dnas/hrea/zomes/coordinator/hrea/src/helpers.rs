use hdk::prelude::*;
use hrea_integrity::*;
use serde::{Deserialize, Serialize};
use serde_json;

/// Merges fields from `latest` into `current`, keeping `current`'s values if they are `Some`.
pub fn merge_fields<T>(current: T, latest: T) -> T
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let mut current_map: serde_json::Map<String, serde_json::Value> = serde_json::to_value(current)
        .unwrap()
        .as_object()
        .unwrap()
        .clone();
    let latest_map: serde_json::Map<String, serde_json::Value> = serde_json::to_value(latest)
        .unwrap()
        .as_object()
        .unwrap()
        .clone();

    for (key, value) in latest_map {
        if !current_map.contains_key(&key) || current_map[&key].is_null() {
            current_map.insert(key, value);
        }
    }

    serde_json::from_value(serde_json::Value::Object(current_map)).unwrap()
}

/// Overlays the non-null fields of a partial update-params struct onto the
/// latest full entry. Unlike `merge_fields`, the two sides may be different
/// types — this is what allows update inputs with all-`Option` fields even
/// when the entry struct has required fields (which would otherwise fail
/// deserialization at the extern boundary on partial payloads).
pub fn merge_partial<P, T>(partial: P, latest: T) -> ExternResult<T>
where
    P: Serialize,
    T: Serialize + for<'de> Deserialize<'de>,
{
    let partial_map: serde_json::Map<String, serde_json::Value> = serde_json::to_value(partial)
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
        .as_object()
        .cloned()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "partial update params must serialize to an object".to_string()
        )))?;
    let mut latest_map: serde_json::Map<String, serde_json::Value> = serde_json::to_value(latest)
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
        .as_object()
        .cloned()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "entry must serialize to an object".to_string()
        )))?;

    for (key, value) in partial_map {
        if !value.is_null() {
            latest_map.insert(key, value);
        }
    }

    serde_json::from_value(serde_json::Value::Object(latest_map))
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))
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
    create_link(new_from_hash, new_to_hash, link_name, tag_prefix)?;
    Ok(())
}

pub fn delete_links(
    from_hash: AnyLinkableHash,
    original_to_hash: AnyLinkableHash,
    link_name: LinkTypes,
) -> ExternResult<()> {
    let tag_prefix = LinkTag(original_to_hash.get_raw_39().to_vec()); // Convert ActionHash to Vec<u8>
    let links = get_links(
        LinkQuery::try_new(from_hash, link_name)?
        .tag_prefix(tag_prefix),
        GetStrategy::Local,
    )?;
    debug!("Links to delete: {:?}", links);
    for link in links {
        delete_link(link.create_link_hash, GetOptions::default())?;
    }
    Ok(())
}

pub(crate) fn try_decode_entry<T>(entry: RecordEntry) -> Result<T, SerializedBytesError>
where
    SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    match entry {
        RecordEntry::Present(entry) => {
            let app_entry_bytes = entry
                .as_app_entry()
                .expect("Expected AppEntryBytes but found None");
            let decoded: T = app_entry_bytes.clone().into_sb().try_into()?;
            Ok(decoded)
        }
        _ => Err(SerializedBytesError::Deserialize(
            "Expected AppEntryBytes but found None".into(),
        )),
    }
}

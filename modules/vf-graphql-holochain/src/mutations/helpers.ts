
import { camelToSnake, snakeToCamel, snakeToCamelString, extractIds } from "../util.js"
import { decode } from "@msgpack/msgpack"
import { encodeHashToBase64 } from "@holochain/client"
import { addEntryToStore, updateLatestRevision } from '../store.js';

export async function createEntry(cell: any, entryType: string, payload: any) {
    const result = await cell.callZome({
        zome_name: 'hrea',
        fn_name: 'create_rea_' + entryType,
        payload: camelToSnake(payload[snakeToCamelString(entryType)]),
    })
    const decoded = decode(result.entry.Present.entry)
    const entry = {
        [entryType]: {
            ...snakeToCamel(decoded),
            id: encodeHashToBase64(result.signed_action.hashed.hash),
            revisionId: encodeHashToBase64(result.signed_action.hashed.hash),
        },
        __typename: entryType.charAt(0).toUpperCase() + entryType.slice(1) + 'Response',
    }
    addEntryToStore(entry[entryType].revisionId, entry[entryType])
    updateLatestRevision(
        entry[entryType].id,
        entry[entryType]
    )
    return entry
}

export async function updateEntry(cell: any, entryType: string, payload: any) {
    const updatePayload = extractIds(payload[entryType])
    const result = await cell.callZome({
        zome_name: 'hrea',
        fn_name: 'update_rea_' + entryType,
        payload: camelToSnake(updatePayload),
    })
    const decoded = decode(result.entry.Present.entry)
    const entry = {
        [entryType]: {
            ...snakeToCamel(decoded),
            revisionId: encodeHashToBase64(result.signed_action.hashed.hash),
            // @ts-ignore
            id: encodeHashToBase64(decoded?.id) || encodeHashToBase64(result.signed_action.hashed.hash),
        },
        __typename: entryType.charAt(0).toUpperCase() + entryType.slice(1) + 'Response',
    }
    addEntryToStore(entry[entryType].revisionId, entry[entryType])
    updateLatestRevision(
        entry[entryType].id,
        entry[entryType]
    )
    return entry
}

export async function deleteEntry(cell: any, typeName: string, args: any) {
    const result = await cell.callZome({
        zome_name: 'hrea',
        fn_name: 'delete_rea_' + typeName,
        payload: args.revisionId,
    });
    if (result === undefined) {
        throw new Error('Failed to delete ' + typeName);
    }
    return true;
}

import { camelToSnake, snakeToCamel, snakeToCamelString, reverseFormatDates, extractIds } from "../util.js"
import { decode } from "@msgpack/msgpack"
import { encodeHashToBase64 } from "@holochain/client"
import { addEntryToStore, updateLatestRevision, removeEntryFromStore } from '../store.js';

export async function createEntry(cell: any, entryType: string, payload: any) {
    const camelCaseEntryType = snakeToCamelString(entryType)
    // if payload is not present, return error
    if (!payload || (!payload[camelCaseEntryType] && !payload.event)) {
        throw new Error(`Payload object or property '${camelCaseEntryType}' is missing`);
    }
    let truePayload;
    if (entryType == 'economic_event') {
        truePayload = {
            event: camelToSnake(reverseFormatDates(payload.event)),
        }
        if (payload.newInventoriedResource) {
            truePayload.new_inventoried_resource = camelToSnake(reverseFormatDates(payload.newInventoriedResource))
        }
    } else {
        truePayload = camelToSnake(reverseFormatDates(payload[camelCaseEntryType]))
    }
    const result = await cell.callZome({
        zome_name: 'hrea',
        fn_name: 'create_rea_' + entryType,
        payload: truePayload,
    })
    const decoded = decode(result.entry.Present.entry)
    const entry = {
        [camelCaseEntryType]: {
            ...snakeToCamel(decoded),
            id: encodeHashToBase64(result.signed_action.hashed.hash),
            revisionId: encodeHashToBase64(result.signed_action.hashed.hash),
            meta: {
                retrievedRevision: {
                    id: encodeHashToBase64(result.signed_action.hashed.hash),
                    time: result.signed_action.hashed.content.timestamp,
                }
            }
        },
        __typename: entryType.charAt(0).toUpperCase() + entryType.slice(1) + 'Response',
    }

    addEntryToStore(entry[camelCaseEntryType].revisionId, entry[camelCaseEntryType])
    updateLatestRevision(
        entry[camelCaseEntryType].id,
        entry[camelCaseEntryType]
    )
    return entry
}

export async function updateEntry(cell: any, entryType: string, payload: any) {
    const camelCaseEntryType = snakeToCamelString(entryType)
        if (!payload || (!payload[camelCaseEntryType] && !payload.event)) {
        throw new Error(`Payload object or property '${camelCaseEntryType}' is missing`);
    }
    let truePayload;
    if (entryType == 'economic_event') {
        truePayload = payload.event
    } else {
        truePayload = payload[camelCaseEntryType]
    }
    const updatePayload = extractIds(truePayload)
    const result = await cell.callZome({
        zome_name: 'hrea',
        fn_name: 'update_rea_' + entryType,
        payload: camelToSnake(reverseFormatDates(updatePayload)),
    })
    const decoded = decode(result.entry.Present.entry)
    const entry = {
        [camelCaseEntryType]: {
            ...snakeToCamel(decoded),
            revisionId: encodeHashToBase64(result.signed_action.hashed.hash),
            // @ts-ignore
            id: encodeHashToBase64(decoded?.id) || encodeHashToBase64(result.signed_action.hashed.hash),
            meta: {
                retrievedRevision: {
                    id: encodeHashToBase64(result.signed_action.hashed.hash),
                    time: result.signed_action.hashed.content.timestamp,
                }
            }
        },
        __typename: entryType.charAt(0).toUpperCase() + entryType.slice(1) + 'Response',
    }
    addEntryToStore(entry[camelCaseEntryType].revisionId, entry[camelCaseEntryType])
    updateLatestRevision(
        entry[camelCaseEntryType].id,
        entry[camelCaseEntryType]
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
    removeEntryFromStore(args.revisionId);
    return true;
}
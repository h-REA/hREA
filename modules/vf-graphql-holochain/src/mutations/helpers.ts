
import { camelToSnake, snakeToCamel, snakeToCamelString, reverseFormatDates, extractIds } from "../util.js"
import { decode } from "@msgpack/msgpack"
import { encodeHashToBase64 } from "@holochain/client"
import { formatResItem } from "../util.js"
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
    const res = await cell.callZome({
        zome_name: 'hrea',
        fn_name: 'create_rea_' + entryType,
        payload: truePayload,
    })
    const formatted = formatResItem(res, encodeHashToBase64(res.signed_action.hashed.hash))
    if (formatted?.revisionId) {
        addEntryToStore(formatted.revisionId, formatted)
        updateLatestRevision(formatted.id, formatted)
    }
    return {
        [camelCaseEntryType]: formatted,
    }
}

export async function updateEntry(cell: any, entryType: string, payload: any) {
    const camelCaseEntryType = snakeToCamelString(entryType)
        if (!payload || (!payload[camelCaseEntryType] && !payload.event && !payload.resource)) {
        throw new Error(`Payload object or property '${camelCaseEntryType}' is missing`);
    }
    let truePayload;
    if (entryType == 'economic_event') {
        truePayload = payload.event
    } else if (entryType == 'economic_resource') {
        truePayload = payload.resource
    } else {
        truePayload = payload[camelCaseEntryType]
    }
    const updatePayload = extractIds(truePayload)
    const updatePayloadWithDates = reverseFormatDates(updatePayload)
    const snakePayload = camelToSnake(updatePayloadWithDates)
    console.log("Updating entry with payload:", snakePayload, entryType, camelCaseEntryType);
    const res = await cell.callZome({
        zome_name: 'hrea',
        fn_name: 'update_rea_' + entryType,
        payload: snakePayload,
    })

    const decoded = decode(res.entry.Present.entry);
    console.log("Response from update:", decoded);
    // @ts-ignore
    const formatted = formatResItem(res, encodeHashToBase64(decoded.id || res.signed_action.hashed.hash))
    console.log("Formatted entry after update:", formatted);
    if (formatted?.revisionId) {
        addEntryToStore(formatted.revisionId, formatted)
        updateLatestRevision(formatted.id, formatted)
    }
    return {
        [camelCaseEntryType]: formatted,
    }
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

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
    // economic_event returns { event, resource? } so EconomicEventResponse can
    // expose the resource created via newInventoriedResource.
    const record = entryType == 'economic_event' ? res.event : res
    const formatted = formatResItem(record, encodeHashToBase64(record.signed_action.hashed.hash))
    if (formatted?.revisionId) {
        addEntryToStore(formatted.revisionId, formatted)
        updateLatestRevision(formatted.id, formatted)
    }
    const response = {
        [camelCaseEntryType]: formatted,
    }
    if (entryType == 'economic_event' && res.resource) {
        const resourceFormatted = formatResItem(res.resource, encodeHashToBase64(res.resource.signed_action.hashed.hash))
        if (resourceFormatted?.revisionId) {
            addEntryToStore(resourceFormatted.revisionId, resourceFormatted)
            updateLatestRevision(resourceFormatted.id, resourceFormatted)
        }
        response.economicResource = resourceFormatted
    }
    return response
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
    const res = await cell.callZome({
        zome_name: 'hrea',
        fn_name: 'update_rea_' + entryType,
        payload: snakePayload,
    })

    const decoded = decode(res.entry.Present.entry);
    // @ts-ignore
    const formatted = formatResItem(res, encodeHashToBase64(decoded.id || res.signed_action.hashed.hash))
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
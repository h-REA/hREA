import { getPaginatedCollection, pluralize, formatResItem, } from "../util.js";
import { addEntryToStore, updateLatestRevision, getLatestRevisionId } from "../store.js";

export async function getOne(cell: any, type: string, args: any) {
    if (!args.id) {
        return null
    }

    // retrieve cached revision unless economic resource
    if (type != "economic_resource") {
        const cachedRevision = getLatestRevisionId(args.id, 10)
        if (cachedRevision) {
            return cachedRevision
        }
    }

    // const cached
    const res = await cell.callZome({
        zome_name: "hrea",
        fn_name: "get_latest_rea_" + type,
        payload: args.id,
    })
    const formatted = formatResItem(res, args.id)
    if (formatted?.revisionId) {
        addEntryToStore(formatted.revisionId, formatted)
        updateLatestRevision(formatted.id, formatted)
    }
    return formatted
}

export async function getAll(cell: any, type: string, args: any) {
    return await getPaginatedCollection(
        cell, 
        "get_all_" + pluralize(type), 
        "get_latest_rea_" + type,
        args
    )
}
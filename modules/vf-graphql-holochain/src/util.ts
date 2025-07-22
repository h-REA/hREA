import { decode, encode } from '@msgpack/msgpack';
import { decodeHashFromBase64, encodeHashToBase64, type ActionHash, type Link } from '@holochain/client'
import { PagingParams } from './types.js'
import { getEntryFromStore, addEntryToStore, updateLatestRevision } from './store.js';

type ExtractIdsOutput = {
  id: string;
  revisionId: string;
  entry: any;
}

type ExtractIdsInput = {
  id: string;
  revisionId: string;
  [key: string]: any;
}

export function extractIds(obj: ExtractIdsInput): ExtractIdsOutput {
  // extract the ids from the object
  let id = obj.id
  let revisionId = obj.revisionId
  let output = {id, revisionId, entry: {}}
  let args = { ...obj } as Partial<typeof obj>
  delete args.id
  delete args.revisionId
  output.entry = args
  return output
}

export async function getPaginatedCollection(cell: any, linkFunc: string, entryFunc: string, paginated: PagingParams) {
  const collection = await getCollection(cell, linkFunc, null, paginated);
  const paginatedCollection = paginateCollection(collection, paginated);
  return paginatedCollection
}
  
export function paginateCollection(collection: any, args: PagingParams) {
  const paginated = collection.map((item: any) => {
      return {
          __typename: 'AgentEdge',
          cursor: item.id,
          node: {
              ...item,
              __typename: item.agentType
          }
      }
  })
  return {
      pageInfo: {
          endCursor: null,
          hasNextPage: false,
          hasPreviousPage: false,
          pageLimit: null,
          startCursor: null,
          totalCount: null,
      },
      edges: paginated
  }
}

export async function getCollectionLinks(
  cell: any, 
  linkFunc: string, 
  payload: any, 
  paginated?: PagingParams
) {
    const result = await cell.callZome({
        zome_name: 'hrea',
        fn_name: linkFunc,
        payload,
    })
    const dedupedLinks: Array<Link> = Array.from(
        result.reduce((map: Map<string, Link>, link: Link) => {
            const encodedTarget = encodeHashToBase64(link.tag);
            if (!map.has(encodedTarget) || (map.get(encodedTarget)?.timestamp ?? 0) < link.timestamp) {
                map.set(encodedTarget, link);
            }
            return map;
        }, new Map()).values()
    );
    return dedupedLinks
}

export async function getCollection(
  cell: any, 
  linkFunc: string, 
  payload: any, 
  paginated?: PagingParams
) {
    const dedupedLinks = await getCollectionLinks(cell, linkFunc, payload, paginated);

    // handle pagination
    if (paginated) {
      const allLinkTags = dedupedLinks.map((link: Link) => encodeHashToBase64(link.tag))
      let sliceBegins = 0
      let sliceEnds = dedupedLinks.length
      if (paginated?.first) {
        sliceBegins = Math.max(paginated?.first - 1, sliceBegins)
      } else if (paginated?.after) {
        sliceBegins = Math.max((allLinkTags.indexOf(paginated?.after || "") + 1), sliceBegins)
      }
      if (paginated?.last) {
        sliceEnds = Math.min(paginated?.last, sliceEnds)
      } else if (paginated?.before) {
        sliceEnds = Math.min(allLinkTags.indexOf(paginated?.before || ""), sliceEnds)
      }
      
      const paginatedLinks = dedupedLinks.slice(sliceBegins, sliceEnds)
      const entries = await getEntries(cell, paginatedLinks);
      return entries
    } else {
      const entries = await getEntries(cell, dedupedLinks);
      return entries
    }
}

export async function getEntries(cell: any, list: Link[]) {
  // for each entry in the list, check the store, and if not found, fetch it
  let entries: any[] = []
  let toFetch: string[] = []

  list.forEach((link) => {
    const revisionId = encodeHashToBase64(link.target)
    let entry = getEntryFromStore(revisionId)
    if (entry) {
      entries.push(entry)
      // console.log("======================got entry from store=======================", entry)
    } else {
      toFetch.push(revisionId)
      // console.log("--------------------------fetching entry from zome---------------------", revisionId)
    }
  })

  if (toFetch.length > 0) {
    let entriesRes = await cell.callZome({
      zome_name: 'hrea',
      fn_name: 'get_generic_entries',
      payload: toFetch,
    })

    entriesRes.forEach((res: any) => {
      if (res?.entry?.Present) {
        const decoded = decode(res.entry.Present.entry)
        const camelCased = snakeToCamel(decoded)
        const withIds = {
          ...camelCased, 
          // @ts-ignore
          id: decoded?.id ? encodeHashToBase64(decoded.id) : encodeHashToBase64(res.signed_action.hashed.hash), 
          revisionId: encodeHashToBase64(res.signed_action.hashed.hash),
          meta: {
            retrievedRevision: {
              id: encodeHashToBase64(res.signed_action.hashed.hash),
              time: res.signed_action.hashed.content.timestamp,
            }
          }
        }
        const withFormattedDates = formatDates(withIds)
        addEntryToStore(withIds.revisionId, withFormattedDates)
        updateLatestRevision(
          withFormattedDates.id,
          withFormattedDates
        )
        entries.push(withFormattedDates)
      } else {
        console.log(`No entry for items (${list})`, res)
      }
    })
  }
  return entries
}

export function formatResItem(resItem: any, id: string) {
  if (!resItem?.entry?.Present?.entry) { return null }
  let decoded = decode(resItem.entry.Present.entry)
  let camel = snakeToCamel(decoded)
  let withId = {
    ...camel, 
    id:  id,
    revisionId: encodeHashToBase64(resItem.signed_action.hashed.hash),
    meta: {
      retrievedRevision: {
        id: encodeHashToBase64(resItem.signed_action.hashed.hash),
        time: resItem.signed_action.hashed.content.timestamp,
      }
    }
  }
  let encoded = formatDates(withId)
  return encoded
}

export function snakeToCamelString(str: string) {
  // convert a snake_case string to camelCase
  return str.replace(/_([a-z])/g, function (g) { return g[1].toUpperCase(); });
}

export function snakeToCamel(lib: any): any {
  // Recursively convert all keys from snake_case to camelCase
  // Do not convert if it's a hash (object with only numeric keys)
  if (Array.isArray(lib)) {
    return lib.map(snakeToCamel);
  } else if (lib !== null && typeof lib === 'object') {
    // Check if all keys are numeric (hash object)
    const keys = Object.keys(lib);
    const allNumeric = keys.length > 0 && keys.every(k => /^\d+$/.test(k));
    if (allNumeric) {
      return lib;
    }
    let camel: any = {};
    for (let key in lib) {
      let camelKey = key.replace(/_([a-z])/g, function (g) { return g[1].toUpperCase(); });
      // convert rea_action
      if (camelKey === "reaAction") {
        camelKey = "action";
      }
      const value = lib[key];
      camel[camelKey] = (typeof value === 'object' && value !== null)
        ? snakeToCamel(value)
        : value;
    }
    return camel;
  }
  return lib;
}

export function camelToSnake(lib: any): any {
  // Recursively convert all keys from camelCase to snake_case
  if (Array.isArray(lib)) {
    return lib.map(camelToSnake);
  } else if (lib !== null && typeof lib === 'object') {
    let snake: any = {};
    for (let key in lib) {
      let snakeKey = key.replace(/([A-Z])/g, "_$1").toLowerCase();
      // convert action
      if (snakeKey === "action") {
        snakeKey = "rea_action";
      }
      // turn null values into undefined
      let value = lib[key] === null ? undefined : lib[key];
      // recursively convert objects and arrays
      if (typeof value === 'object' && value !== null) {
        value = camelToSnake(value);
      }
      snake[snakeKey] = value;
    }
    return snake;
  }
  return lib;
}

export function detectQuantityValueField(field: any) {
  if (field.has_numerical_value !== undefined && field.has_unit !== undefined) {
    return true
  }
  return false
}

export function decodePotentialQuantityValueField(field: any) {
  if (field.hasNumericalValue !== undefined && field.hasUnit !== undefined) {
    return {
      has_numerical_value: field.hasNumericalValue,
      has_unit: field.hasUnit?.id ? field.hasUnit.id : (field.hasUnit ? field.hasUnit : null)
    }
  }
  return field
}

export function findAndDecodeQuantityValueFields(obj: any) {
  for (let key in obj) {
    if (typeof obj[key] === 'object' && obj[key] !== null) {
      if (obj[key].hasNumericalValue !== undefined && obj[key].hasUnit !== undefined) {
        obj[key] = decodePotentialQuantityValueField(obj[key])
      }
    }
  }
  return obj
}

// const dateFields = ['due', 'hasBeginning', 'hasEnd', 'hasPointInTime']
// export function formatDates(obj: any) {
//   dateFields.forEach(field => {
//     if (obj[field]) {
//       obj[field] = new Date(obj[field] / 1000)
//     }
//   })
//   return obj
// }

// export function reverseFormatDates(obj: any) {
//   dateFields.forEach(field => {
//     if (obj[field]) {
//       obj[field] = new Date(obj[field]).getTime() * 1000
//     }
//   })
//   return obj
// }
const dateFields = ['due', 'hasBeginning', 'hasEnd', 'hasPointInTime']
export function formatDates(obj: any) {
  dateFields.forEach(field => {
    if (obj[field]) {
      // Only convert if not already a Date
      if (!(obj[field] instanceof Date)) {
        // if it is a number, assume it's a timestamp
        if (typeof obj[field] === 'number') {
          obj[field] = new Date(obj[field] / 1000)
        } else if (typeof obj[field] === 'string') {
          // if it's a string, try to parse it as a date
          const parsedDate = new Date(obj[field])
          if (!isNaN(parsedDate.getTime())) {
            obj[field] = parsedDate
          }
        }
      }
    }
  })
  return obj
}

export function reverseFormatDates(obj: any) {
  dateFields.forEach(field => {
    if (obj[field]) {
      // Only convert if not already a number (timestamp)
      if (obj[field] instanceof Date) {
        obj[field] = obj[field].getTime() * 1000
      } else if (typeof obj[field] === 'string') {
        // if it's a string, try to parse it as a date
        const parsedDate = new Date(obj[field])
        if (!isNaN(parsedDate.getTime())) {
          obj[field] = parsedDate.getTime() * 1000
        }
      }
    }
  })
  return obj
}

export function pluralize(str: string) {
  if (str === 'person') {
    return 'people'
  } else if (str.endsWith('s')) {
    return str + 'es'
  } else if (str.endsWith('y')) {
    return str.slice(0, -1) + 'ies'
  } else if (str.endsWith('o')) {
    return str + 'es'
  } else if (str.endsWith('ch')) {
    return str + 'es'
  } else if (str.endsWith('sh')) {
    return str + 'es'
  } else {
    return str + 's'
  }
}

export function decodeHashFields(obj: any, hashFields: string[]) {
  hashFields.forEach(field => {
    if (obj[field]) {
      const decodedField = decodeHashFromBase64(obj[field])
      obj[field] = decodedField
      obj[`${field}Id`] = decodedField
    }
  })
  return obj
}

const actions = {
  "dropoff": {
    id: "dropoff",
    label: "dropoff",
    resourceEffect: "decrement",
    onhandEffect: "decrement",
    inputOutput: "output",
    pairsWith: "pickup"
  },
  "pickup": {
    id: "pickup",
    label: "pickup",
    resourceEffect: "increment",
    onhandEffect: "increment",
    inputOutput: "input",
    pairsWith: "dropoff"
  },
  "consume": {
    id: "consume",
    label: "consume",
    resourceEffect: "decrement",
    onhandEffect: "decrement",
    inputOutput: "input",
    pairsWith: "notApplicable"
  },
  "use": {
    id: "use",
    label: "use",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "input",
    pairsWith: "notApplicable"
  },
  "work": {
    id: "work",
    label: "work",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "input",
    pairsWith: "notApplicable"
  },
  "cite": {
    id: "cite",
    label: "cite",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "input",
    pairsWith: "notApplicable"
  },
  "produce": {
    id: "produce",
    label: "produce",
    resourceEffect: "increment",
    onhandEffect: "increment",
    inputOutput: "output",
    pairsWith: "notApplicable"
  },
  "accept": {
    id: "accept",
    label: "accept",
    resourceEffect: "noEffect",
    onhandEffect: "decrement",
    inputOutput: "input",
    pairsWith: "modify"
  },
  "modify": {
    id: "modify",
    label: "modify",
    resourceEffect: "noEffect",
    onhandEffect: "increment",
    inputOutput: "output",
    pairsWith: "accept"
  },
  "pass": {
    id: "pass",
    label: "pass",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "output",
    pairsWith: "accept"
  },
  "fail": {
    id: "fail",
    label: "fail",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "output",
    pairsWith: "accept"
  },
  "deliver-service": {
    id: "deliver-service",
    label: "deliver-service",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "output",
    pairsWith: "notApplicable"
  },
  "transfer-all-rights": {
    id: "transfer-all-rights",
    label: "transfer-all-rights",
    resourceEffect: "decrementIncrement",
    onhandEffect: "noEffect",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "transfer-custody": {
    id: "transfer-custody",
    label: "transfer-custody",
    resourceEffect: "noEffect",
    onhandEffect: "decrementIncrement",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "transfer": {
    id: "transfer",
    label: "transfer",
    resourceEffect: "decrementIncrement",
    onhandEffect: "decrementIncrement",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "move": {
    id: "move",
    label: "move",
    resourceEffect: "decrementIncrement",
    onhandEffect: "decrementIncrement",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "raise": {
    id: "raise",
    label: "raise",
    resourceEffect: "increment",
    onhandEffect: "increment",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "lower": {
    id: "lower",
    label: "lower",
    resourceEffect: "decrement",
    onhandEffect: "decrement",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  }
}

export function getAction(id: string) {
  return actions[id]
}
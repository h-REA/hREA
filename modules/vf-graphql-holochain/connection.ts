/**
 * Connection wrapper for Holochain DNA method calls
 *
 * @package: Holo-REA
 * @since:   2019-05-20
 */

import { AppSignalCb, AppWebsocket, CellId } from '@holochain/conductor-api'
import deepForEach from 'deep-for-each'
import { format, parse } from 'fecha'
import { Base64 } from "js-base64"
import { DNAIdMappings } from './types'


//----------------------------------------------------------------------------------------------------------------------
// Connection persistence and multi-conductor / multi-agent handling
//----------------------------------------------------------------------------------------------------------------------

let DEFAULT_CONNECTION_URI = process.env.REACT_APP_HC_CONN_URL || ''
const CONNECTION_CACHE: { [i: string]: Promise<AppWebsocket> } = {}

/**
 * Inits a connection for the given websocket URI. If no `socketURI` is provided,
 * a connection is attempted via the `REACT_APP_HC_CONN_URL` environment variable.
 *
 * This method gives calling code an opportunity to register globals for all future
 * instances of a connection of the same `socketURI`. To ensure this is done reliably,
 * a runtime error will be thrown by `getConnection` if no `openConnection` has
 * been previously performed for the same `socketURI`.
 */
export const openConnection = (socketURI: string, traceAppSignals?: AppSignalCb) => {
  if (!socketURI) {
    socketURI = DEFAULT_CONNECTION_URI
  }
  if (!socketURI) {
    throw new Error('Holochain socket URI not specified!')
  }

  console.log(`Init Holochain connection: ${socketURI}`)

  CONNECTION_CACHE[socketURI] = AppWebsocket.connect(socketURI, undefined, traceAppSignals)
    .then((client) => {
        console.log(`Holochain connection to ${socketURI} OK`)
        return client
      })

  return CONNECTION_CACHE[socketURI]
}

const getConnection = (socketURI: string) => {
  if (!CONNECTION_CACHE[socketURI]) {
    throw new Error(`Connection for ${socketURI} not initialised! Please call openConnection() first.`)
  }

  return CONNECTION_CACHE[socketURI]
}


//----------------------------------------------------------------------------------------------------------------------
// Holochain / GraphQL type translation layer
//----------------------------------------------------------------------------------------------------------------------

// @see https://github.com/holochain-open-dev/core-types/blob/main/src/utils.ts
function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1));
}

// @see https://github.com/holochain-open-dev/core-types/blob/main/src/utils.ts
function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

/**
 * Decode raw data input coming from Holochain API websocket.
 *
 * Mutates in place- we have no need for the non-normalised primitive format and this saves memory.
 */
const decodeFields = (cellDNAHash: Buffer, result: any): void => {
  coerceIdFields(cellDNAHash, result)
  decodeDateFields(result)
}

// check toplevel result objects for ID field buffers and coerce to strings for GraphQL
const coerceIdFields = (cellDNAHash: Buffer, result: any): void => {
  let cellIdStr = serializeHash(cellDNAHash)
  let r
  for (let field of Object.keys(result)) {
    r = result[field]
    if (r.id) {
      r.id = `${cellIdStr}/${serializeHash(r.id)}`
    }
    if (r.revisionId) {
      r.revisionId = `${cellIdStr}/${serializeHash(r.revisionId)}`
    }
  }
}

const isoDateRegex = /^\d{4}-\d\d-\d\d(T\d\d:\d\d:\d\d(\.\d\d\d)?)?([+-]\d\d:\d\d)?$/

// recursively check for Date strings and convert to JS date objects upon receiving
const decodeDateFields = (result: any): void => {
  deepForEach(result, (value, prop, subject, path) => {
    if (value.match(isoDateRegex)) {
      subject[prop] = parse(value, 'isoDateTime')
    }
  })
}

/**
 * Encode application runtime data into serialisable format for transmitting to API websocket.
 *
 * Clones data in order to keep input data pristine.
 */
const encodeFields = (cellDNAHash: Buffer, args: any): any => {
  args = encodeIdFields(cellDNAHash, args)
  args = encodeDateFields(args)
  return args
}

// decode string versions of ID fields into Buffer objects for Holochain
const encodeIdFields = (cellDNAHash: Buffer, args: any): any => {
  const res = {}
  let r
  // pull toplevel parameter ID info out of string values into simplified buffers before sending
  for (let field of Object.keys(args)) {
    r = args[field]
    if (r.revisionId) {
      const [cellId, revisionId] = parseSingleIdField(r.revisionId)
      if (cellId !== cellDNAHash) {
        throw new Error(`Record data from ${r.revisionId} passed to incorrect cell ${cellDNAHash}`)
      }
      r.revisionId = revisionId
    }
    if (r.id) {
      const [cellId, id] = parseSingleIdField(r.id)
      if (cellId !== cellDNAHash) {
        throw new Error(`Record data from ${r.id} passed to incorrect cell ${cellDNAHash}`)
      }
      r.id = id
    }
    res[field] = r
  }
}

// recursively check for Date objects and coerce to ISO8601 strings for transmission
const encodeDateFields = (args: any): any => {
  const res = {}
  deepForEach(res, (value, prop, subject, path) => {
    if (value instanceof Date) {
      subject[prop] = format(value, 'isoDateTime')
    }
  })

  return res
}

const parseSingleIdField = (field: string): CellId => {
  const matches = field.split('/')
  return [
    Buffer.from(deserializeHash(matches[1])),
    Buffer.from(deserializeHash(matches[2])),
  ]
}


//----------------------------------------------------------------------------------------------------------------------
// Holochain cell API method binding API
//----------------------------------------------------------------------------------------------------------------------

// explicit type-loss at the boundary
export type BoundZomeFn = (args: any) => any;

/**
 * Higher-order function to generate async functions for calling zome RPC methods
 */
const zomeFunction = (socketURI: string, cell_id: CellId, zome_name: string, fn_name: string): BoundZomeFn => async (args) => {
  const { callZome } = await getConnection(socketURI)
  const res = await callZome({
    cap: null, // :TODO:
    cell_id,
    zome_name,
    fn_name,
    provenance: cell_id[1],
    payload: encodeFields(cell_id[0], args),
  })
  decodeFields(cell_id[0], res)
  return res
}

/**
 * External API for accessing zome methods, passing them through an optional intermediary DNA ID mapping
 *
 * @param mappings  DNAIdMappings to use for this collaboration space.
 *                  `instance` must be present in the mapping, and the mapped CellId will be used instead of `instance` itself.
 * @param socketURI If provided, connects to the Holochain conductor on a different URI.
 *
 * @return bound async zome function which can be called directly
 */
export const mapZomeFn = (mappings: DNAIdMappings, socketURI: string, instance: string, zome: string, fn: string) =>
  zomeFunction(socketURI, (mappings && mappings[instance]), zome, fn)

/**
 * Connection wrapper for Holochain DNA method calls
 *
 * :TODO: :WARNING:
 *
 * This layer is currently unsuitable for mixing with DNAs that use dna-local identifier formats, and
 * will cause encoding errors if 2-element lists of identifiers are passed.
 *
 * Such tuples are interpreted as [`DnaHash`, `AnyDhtHash`] pairs by the GraphQL <-> Holochain
 * serialisation layer and transformed into compound IDs at I/O time. So, this adapter should
 * *only* be used to wrap DNAs explicitly developed with multi-DNA references in mind.
 *
 * Also :TODO: - standardise a binary format for universally unique Holochain entry/header identifiers.
 *
 * @package: Holo-REA
 * @since:   2019-05-20
 */

import { AppSignalCb, AppWebsocket, CellId, HoloHash } from '@holochain/conductor-api'
import deepForEach from 'deep-for-each'
import isObject from 'is-object'
import { format, parse } from 'fecha'
import { Base64 } from "js-base64"
import { DNAIdMappings } from './types'

type RecordId = [HoloHash, HoloHash]

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

const HOLOCHAIN_IDENTIFIER_LEN = 39
const idMatchRegex = /^[A-Za-z0-9_+\-/]{53}={0,2}:[A-Za-z0-9_+\-/]{53}={0,2}$/

// @see https://github.com/holochain-open-dev/core-types/blob/main/src/utils.ts
function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1))
}

function deserializeId(field: string): RecordId {
  const matches = field.split(':')
  return [
    Buffer.from(deserializeHash(matches[0])),
    Buffer.from(deserializeHash(matches[1])),
  ]
}

// @see https://github.com/holochain-open-dev/core-types/blob/main/src/utils.ts
function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`
}

function seralizeId(id: RecordId): string {
  return `${serializeHash(id[0])}:${serializeHash(id[1])}`
}

const LONG_DATETIME_FORMAT = 'YYYY-MM-DDTHH:mm:ss.SSSZ'
const isoDateRegex = /^\d{4}-\d\d-\d\d(T\d\d:\d\d:\d\d(\.\d\d\d)?)?([+-]\d\d:\d\d)?$/

/**
 * Decode raw data input coming from Holochain API websocket.
 *
 * Mutates in place- we have no need for the non-normalised primitive format and this saves memory.
 */
const decodeFields = (cellDNAHash: Buffer, result: any): void => {
  deepForEach(result, (value, prop, subject) => {

    // Match 2-element arrays of Buffer objects as IDs. Format conflicts not expected, but... :SHONK:
    if (Array.isArray(value) && value.length == 2 &&
      value[0] instanceof Buffer && value[1] instanceof Buffer &&
      value[0].length === HOLOCHAIN_IDENTIFIER_LEN && value[1].length === HOLOCHAIN_IDENTIFIER_LEN)
    {
      subject[prop] = seralizeId(value as RecordId)
    }

    // recursively check for Date strings and convert to JS date objects upon receiving
    if (value.match && value.match(isoDateRegex)) {
      subject[prop] = parse(value, LONG_DATETIME_FORMAT)
    }

  })
}

/**
 * Encode application runtime data into serialisable format for transmitting to API websocket.
 *
 * Clones data in order to keep input data pristine.
 */
const encodeFields = (cellDNAHash: Buffer, args: any): any => {
  let res = args

  // encode dates as ISO8601 DateTime strings
  if (args instanceof Date) {
    return format(args, LONG_DATETIME_FORMAT)
  }

  // deserialise any identifiers back to their binary format
  else if (args.match && args.match(idMatchRegex)) {
    return deserializeId(args)
  }

  // recurse into child fields
  else if (Array.isArray(args)) {
    res = []
    args.forEach((value, key) => {
      res[key] = encodeFields(cellDNAHash, value)
    })
  } else if (isObject(args)) {
    res = {}
    for (const key in args) {
      res[key] = encodeFields(cellDNAHash, args[key])
    }
  }

  return res
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

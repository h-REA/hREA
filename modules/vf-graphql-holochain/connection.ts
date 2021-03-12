/**
 * Connection wrapper for Holochain DNA method calls
 *
 * :TODO: provide a facility for this to be wired into Redux, allow
 * multiple connection sources & generally be more modular
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { AppSignalCb, AppWebsocket, CellId } from '@holochain/conductor-api'
import deepForEach from 'deep-for-each'
import { format } from 'fecha'
import { DNAIdMappings } from './types'

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

// check toplevel result objects for ID fields and coerce to strings for GraphQL
const decodeIdFields = (result: any): void => {
  let r
  for (let field of Object.keys(result)) {
    r = result[field]
    if (r.cellId) {
      if (r.id) {
        r.id = `${r.cellId.toString('base64')}/${r.id.toString('base64')}`
      }
      if (r.revisionId) {
        r.revisionId = `${r.cellId.toString('base64')}/${r.revisionId.toString('base64')}`
      }
    }
  }
}

const encodeFields = (args: any): any => {
  const res = {}
  let r
  // pull toplevel parameter ID info out of string values into buffers before sending
  for (let field of Object.keys(args)) {
    r = args[field]
    if (r.revisionId) {
      const [cellId, revisionId] = decodeSingleIdField(r.revisionId)
      // :WARNING: presuming correctness and association here
      r.cellId = cellId
      r.revisionId = revisionId
    }
    if (r.id) {
      const [cellId, id] = decodeSingleIdField(r.id)
      // :WARNING: presuming correctness and association here
      r.cellId = cellId
      r.id = id
    }
    res[field] = r
  }

  // convert all dates to ISO8601 strings
  deepForEach(res, (value, prop, subject, path) => {
    if (value instanceof Date) {
      subject[prop] = format(value, 'isoDateTime')
    }
  })

  return res
}

type BufferArr = [Buffer, Buffer]
const decodeSingleIdField = (field: string): BufferArr => {
  const matches = field.match(/^(\w+)\/(\w+)$/)
  if (!matches) throw new Error('Format error in ID field value')
  return [
    Buffer.from(matches[1], 'base64'),
    Buffer.from(matches[2], 'base64'),
  ]
}

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
    payload: encodeFields(args), // clones to keep input data pristine
  })
  decodeIdFields(res) // mutates in-place to save memory
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

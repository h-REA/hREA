/**
 * Connection wrapper for Holochain DNA method calls
 *
 * :TODO: provide a facility for this to be wired into Redux, allow
 * multiple connection sources & generally be more modular
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { AppWebsocket, CellId } from '@holochain/conductor-api'
import { DNAIdMappings } from './types'

type ConnURI = { url: string } | undefined

type Call = (...segments: Array<string>) => (params: any) => Promise<any>
type CallZome = (instanceId: string, zome: string, func: string) => (params: any) => Promise<any>
type OnSignal = (callback: (params: any) => void) => void
type Close = () => Promise<any>

let DEFAULT_CONNECTION_URI: ConnURI = process.env.REACT_APP_HC_CONN_URL ? { url: process.env.REACT_APP_HC_CONN_URL } : undefined
const CONNECTION_CACHE: { [i: string]: Promise<AppWebsocket> } = {}

/**
 * For use by external scripts to hook a default connection URI prior to initialisation.
 *
 * To bind per-instance connection URIs, the recommended pattern is to now use
 * the optional parameter to mapZomeFn(), passing DNA mappings from a higher-order
 * function in order to allow runtime rebinding.
 */
export function setConnectionURI (url: string): void {
  DEFAULT_CONNECTION_URI = { url }
}

/**
 * Inits a connection for the given websocket URI. If none is provided, a default
 * connection is attempted either by `REACT_APP_HC_CONN_URL` environment variable
 * or the built-in Holochain conductor resolution if the app is run from within
 * a Holochain conductor.
 */
const BASE_CONNECTION = (socketURI: ConnURI = undefined) => {
  if (!socketURI) {
    socketURI = DEFAULT_CONNECTION_URI
  }
  if (!socketURI) {
    throw new Error('Holochain socket URI not specified!')
  }
  const connId = socketURI.url

  if (CONNECTION_CACHE[connId]) {
    return CONNECTION_CACHE[connId]
  }

  console.log(`Init Holochain connection: ${connId}`)

  CONNECTION_CACHE[connId] = AppWebsocket.connect(connId)
    .then((client) => {
        console.log(`Holochain connection to ${connId} OK`)
        return client
      })

  return CONNECTION_CACHE[connId]
}

export interface ZomeFnOpts {
  resultParser?: (resp: any) => any
}

// :TODO: remove this someday, @see https://github.com/graphql/graphql-js/pull/2910
function decodeBuffers (data) {
  const newData = {}
  Object.keys(data).forEach(k => {
    if (data[k] instanceof Buffer) {
      newData[k] = data[k].toString('hex')
    } else if (typeof data[k] === 'object') {
      newData[k] = decodeBuffers(data[k])
    } else {
      newData[k] = data[k]
    }
  })
  return newData
}

/**
 * Higher-order function to generate async functions for calling zome RPC methods
 */
const zomeFunction = (cell_id: CellId, zome_name: string, fn_name: string, socketURI: ConnURI = undefined) => async (args: any, opts: ZomeFnOpts = {}) => {
  const { callZome } = await BASE_CONNECTION(socketURI)

  const res = await callZome({
    cap: null, // :TODO:
    cell_id,
    zome_name,
    fn_name,
    provenance: cell_id[1],
    payload: args,
  })


  return decodeBuffers(res)
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
export const mapZomeFn = (mappings: DNAIdMappings, socketURI: string | undefined, instance: string, zome: string, fn: string) =>
  zomeFunction((mappings && mappings[instance]) || instance, zome, fn, socketURI ? { url: socketURI } : undefined)

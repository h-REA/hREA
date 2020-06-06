/**
 * Connection wrapper for Holochain DNA method calls
 *
 * :TODO: provide a facility for this to be wired into Redux, allow
 * multiple connection sources & generally be more modular
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { connect } from '@holochain/hc-web-client'
import { DNAIdMappings } from './types'

type ConnURI = { url: string } | undefined

type Call = (...segments: Array<string>) => (params: any) => Promise<any>
type CallZome = (instanceId: string, zome: string, func: string) => (params: any) => Promise<any>
type OnSignal = (callback: (params: any) => void) => void
type Close = () => Promise<any>
type CachedConnection = { call: Call, callZome: CallZome, close: Close, onSignal: OnSignal, ws: any }

let DEFAULT_CONNECTION_URI: ConnURI = process.env.REACT_APP_HC_CONN_URL ? { url: process.env.REACT_APP_HC_CONN_URL } : undefined
const CONNECTION_CACHE: { [i: string]: Promise<CachedConnection> } = {}

/**
 * For use by external scripts which need to init multiple connections.
 * You can call this method prior to triggering a `zomeFunction` to ensure
 * that the call will hit the specified websocket URI.
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
  const connId = socketURI ? socketURI.url : '__default__'

  if (CONNECTION_CACHE[connId]) {
    return CONNECTION_CACHE[connId]
  }

  console.log(`Init Holochain connection: ${socketURI ? socketURI.url : '<default>'}`)

  CONNECTION_CACHE[connId] = connect(socketURI)
    .then(conn => {
      console.log(`Holochain connection OK: ${socketURI ? socketURI.url : '<default>'}`)
      return conn
    })

  return CONNECTION_CACHE[connId]
}

export interface ZomeFnOpts {
  resultParser?: (resp: any) => any
}

/**
 * Higher-order function to generate async functions for calling zome RPC methods
 */
const zomeFunction = (instance: string, zome: string, fn: string, socketURI: ConnURI = undefined) => async (args: any, opts: ZomeFnOpts = {}) => {
  const { callZome } = await BASE_CONNECTION(socketURI)
  const zomeCall = callZome(instance, zome, fn)

  const rawResult = await zomeCall(args)
  const jsonResult = JSON.parse(rawResult)
  let error = jsonResult['Err'] || jsonResult['SerializationError']

  // deal with complex error responses
  if (!(typeof error === 'string' || error instanceof String)) {
    error = JSON.stringify(error)
  }

  if (error) throw new Error(error)

  const rawOk = jsonResult['Ok']
  return opts.resultParser ? opts.resultParser(rawOk) : rawOk
}

/**
 * External API for accessing zome methods, passing them through an optional intermediary DNA ID mapping
 *
 * @param mappings  DNAIdMappings to use for this collaboration space.
 *                  If `instance` is present in the mapping, the mapped DNA ID is used instead of `instance` itself.
 * @param socketURI If provided, connects to the Holochain conductor on a different URI.
 *
 * @return bound async zome function which can be called directly
 */
export const mapZomeFn = (mappings: DNAIdMappings, socketURI: string | undefined, instance: string, zome: string, fn: string) =>
  zomeFunction((mappings && mappings[instance]) || instance, zome, fn, socketURI ? { url: socketURI } : undefined)

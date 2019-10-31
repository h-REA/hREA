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

let DEFAULT_CONNECTION_URI = process.env.REACT_APP_HC_CONN_URL ? { url: process.env.REACT_APP_HC_CONN_URL } : undefined
const CONNECTION_CACHE = {}

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
const BASE_CONNECTION = (socketURI = undefined) => {
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
export const zomeFunction = (instance, zome, fn, socketURI = undefined) => async (args, opts: ZomeFnOpts = {}) => {
  const { callZome } = await BASE_CONNECTION(socketURI)
  const zomeCall = callZome(instance, zome, fn)

  const rawResult = await zomeCall(args)
  const jsonResult = JSON.parse(rawResult)
  const error = jsonResult['Err'] || jsonResult['SerializationError']
  const rawOk = jsonResult['Ok']

  if (error) throw (error)
  return opts.resultParser ? opts.resultParser(rawOk) : rawOk
}

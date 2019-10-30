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

const CONNECTION_CACHE = {}

const BASE_CONNECTION = (socketURI = undefined) => {
  if (!socketURI) {
    socketURI = process.env.REACT_APP_HC_CONN_URL ? { url: process.env.REACT_APP_HC_CONN_URL } : undefined
  }
  const connId = socketURI || '__default__'

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

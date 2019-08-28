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

console.log(`attempt connection at ${process.env.REACT_APP_HC_CONN_URL || '<default>'}`)

const BASE_CONNECTION = connect(process.env.REACT_APP_HC_CONN_URL || undefined)

export interface ZomeFnOpts {
  resultParser?: (resp: any) => any
}

export const zomeFunction = async (instance, zome, fn) => async (args, opts: ZomeFnOpts = {}) => {
  const { callZome } = await BASE_CONNECTION
  const zomeCall = callZome(instance, zome, fn)

  const rawResult = await zomeCall(args)
  const jsonResult = JSON.parse(rawResult)
  const error = jsonResult['Err'] || jsonResult['SerializationError']
  const rawOk = jsonResult['Ok']

  if (error) throw (error)
  return opts.resultParser ? opts.resultParser(rawOk) : rawOk
}

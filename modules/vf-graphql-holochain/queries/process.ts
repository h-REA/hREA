/**
 * Top-level queries relating to Processes
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'

import {
  Process,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProcess = zomeFunction('observation', 'process', 'get_process')

// Read a single record by ID
export const process = async (root, args): Promise<Process> => {
  return (await readProcess({ address: args.id })).process
}

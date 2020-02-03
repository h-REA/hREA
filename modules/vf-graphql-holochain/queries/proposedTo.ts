/**
 * Top-level queries relating to ProposedTo
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'

import {
  ProposedTo,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProposedTo = zomeFunction('specification', 'process_specification', 'get_process_specification')

// Read a single record by ID
export const proposedTo = async (root, args): Promise<ProposedTo> => {
  return (await readProposedTo({ address: args.id })).proposedTo
}

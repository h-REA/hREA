/**
 * Top-level queries relating to Proposal
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'

import {
  Proposal,
  ProposedTo,
  ProposedIntent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProposal = zomeFunction('proposal', 'proposal', 'get_proposal')

// Read a single record by ID
export const proposal = async (root, args): Promise<Proposal> => {
  return (await readProposal({ address: args.id })).proposal
}

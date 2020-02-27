/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'

import {
  Proposal,
  ProposedIntent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProposal = zomeFunction('proposal', 'proposal', 'get_proposal')

export const publishedIn = async (record: ProposedIntent): Promise<Proposal> => {
  return (await readProposal({address:record.publishedIn})).proposal
}

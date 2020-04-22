/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'

import {
  Proposal,
  Intent,
  ProposedIntent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProposal = zomeFunction('proposal', 'proposal', 'get_proposal')
const readIntent = zomeFunction('planning', 'intent', 'get_intent')

export const publishedIn = async (record: ProposedIntent): Promise<Proposal> => {
  return (await readProposal({address:record.publishedIn})).proposal
}

export const publishes = async (record: ProposedIntent): Promise<Intent> => {
  return (await readIntent({address:record.publishes})).intent
}

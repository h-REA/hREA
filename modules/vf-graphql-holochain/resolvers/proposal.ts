/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'

import {
  Proposal,
  ProposedTo,
  ProposedIntent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProposedTo = zomeFunction('proposal', 'proposed_to', 'get_proposed_to')
const readProposedIntent = zomeFunction('proposal', 'proposed_intent', 'get_proposed_intent')

const extractProposedTo = (data): ProposedTo => data.proposedTo
const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export const publishes = async (record: Proposal): Promise<ProposedIntent[]> => {
  return (await Promise.all((record.publishes || []).map((address)=>readProposedIntent({address})))).map(extractProposedIntent)
}

export const publishedTo = async (record: Proposal): Promise<ProposedTo[]> => {
  return (await Promise.all((record.publishedTo || []).map((address)=>readProposedTo({address})))).map(extractProposedTo)
}

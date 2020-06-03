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
  Agent,
} from '@valueflows/vf-graphql'

import { agent as loadAgent } from '../queries/agent'

// :TODO: how to inject DNA identifier?
const readProposal = zomeFunction('proposal', 'proposal', 'get_proposal')

export const proposedTo = async (record: ProposedTo): Promise<Agent> => {
  return loadAgent(record, { id: record.proposedTo })
}

export const proposed = async (record: ProposedTo): Promise<Proposal> => {
  return (await readProposal({address:record.proposed})).proposal
}

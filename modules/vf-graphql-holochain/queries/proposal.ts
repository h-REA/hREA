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
const readProposedTo = zomeFunction('proposal', 'proposed_to', 'get_proposed_to')
const readProposedIntent = zomeFunction('proposal', 'proposed_intent', 'get_proposed_intent')

const getProposedTo = (address): Promise<ProposedTo> => readProposedTo({address})
const getProposedIntent = (address): Promise<ProposedIntent> => readProposedIntent({address})

const extractProposedTo = (data): ProposedTo => data.proposedTo
const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

// Read a single record by ID
export const proposal = async (root, args): Promise<Proposal> => {
  let proposal = (await readProposal({ address: args.id })).proposal
  proposal.publishedTo = (await Promise.all(proposal.publishedTo.map(addr=>getProposedTo(addr)))).map(extractProposedTo)
  proposal.publishes = (await Promise.all(proposal.publishes.map(addr=>getProposedIntent(addr)))).map(extractProposedIntent)
  return proposal
}

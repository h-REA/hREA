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
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProposal = zomeFunction('proposal', 'proposal', 'get_proposal')
const readProposedTo = zomeFunction('proposal', 'proposed_to', 'get_proposed_to')

const getProposedTo = (address): Promise<ProposedTo> => readProposedTo({address})

const extractProposedTo = (data): ProposedTo => data.proposedTo

// Read a single record by ID
export const proposal = async (root, args): Promise<Proposal> => {
  let proposal = (await readProposal({ address: args.id })).proposal
  proposal.publishedTo = (await Promise.all(proposal.publishedTo.map(addr=>getProposedTo(addr)))).map(extractProposedTo)
  return proposal
}

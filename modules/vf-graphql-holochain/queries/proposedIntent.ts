/**
 * Top-level queries relating to ProposedIntent
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'

import {
  ProposedIntent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProposedIntent = zomeFunction('proposal', 'proposed_intent', 'get_proposed_intent')

// Read a single record by ID
export const proposedIntent = async (root, args): Promise<ProposedIntent> => {
  return (await readProposedIntent({ address: args.id })).proposedIntent
}

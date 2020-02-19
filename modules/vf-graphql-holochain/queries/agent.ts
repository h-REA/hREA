/**
 * Agent queries
 *
 * @package: Holo-REA
 * @since:   2020-02-19
 */

import { zomeFunction } from '../connection'

import {
  Agent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const read = zomeFunction('agent', 'agent', 'get_my_agent')

export const myAgent = async (root, args): Promise<Agent> => {
  return read({})
}

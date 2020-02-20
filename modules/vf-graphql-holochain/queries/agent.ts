/**
 * Agent queries
 *
 * @package: Holo-REA
 * @since:   2020-02-19
 */

import { zomeFunction } from '../connection'
import { injectTypename } from '../types'

import {
  Agent
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const read = zomeFunction('agent', 'agent', 'get_my_agent')

// :TODO: is myAgent always a 'Person' in Holochain, or will we allow users to act in an Organization context directly?
export const myAgent = injectTypename('Person', async (root, args): Promise<Agent> => {
  const agentData: Agent = (await read({})).agent
  // :TODO: wire to Personas hApp
  agentData['name'] = `Agent ${agentData.id.substr(0, 4)}`
  return agentData
})

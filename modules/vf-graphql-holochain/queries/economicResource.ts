/**
 * Top-level queries relating to Economic Resources
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { zomeFunction } from '../connection'

import {
  EconomicResource,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readResource = zomeFunction('observation', 'economic_resource', 'get_resource')

// Read a single record by ID
export const economicResource = async (root, args): Promise<EconomicResource> => {
  return (await readResource({ address: args.id })).economicResource
}

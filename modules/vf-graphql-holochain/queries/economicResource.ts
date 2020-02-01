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
const readOne = zomeFunction('observation', 'economic_resource', 'get_resource')
const readAll = zomeFunction('observation', 'economic_resource', 'get_all_resources')

// Read a single record by ID
export const economicResource = async (root, args): Promise<EconomicResource> => {
  return (await readOne({ address: args.id })).economicResource
}

export const allEconomicResources = async (root, args): Promise<[EconomicResource]> => {
  return (await readAll()).map(e => e.economicResource)
}

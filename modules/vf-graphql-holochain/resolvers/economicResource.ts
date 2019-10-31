/**
 * Resolvers for EconomicResource fields
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { zomeFunction } from '../connection'

import {
  EconomicResource,
} from '@valueflows/vf-graphql'

const readResources = zomeFunction('observation', 'economic_resource', 'query_resources')

export const containedIn = async (record: EconomicResource): Promise<EconomicResource> => {
  return (await readResources({ params: { contains: record.id } })).pop()['economicResource']
}

export const contains = async (record: EconomicResource): Promise<[EconomicResource]> => {
  return (await readResources({ params: { containedIn: record.id } })).map(({ economicResource }) => economicResource)
}

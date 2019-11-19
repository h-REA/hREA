/**
 * Resolvers for ResourceSpecification fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'

import {
  EconomicResource,
  ResourceSpecification,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readConforming = zomeFunction('observation', 'resource_specification', 'query_resource_specification')

export const conformingResources = async (record: ResourceSpecification): Promise<[EconomicResource]> => {
  return (await readConforming({ params: { conformingResources: record.id } })).map(({ economicResource }) => economicResource )
}

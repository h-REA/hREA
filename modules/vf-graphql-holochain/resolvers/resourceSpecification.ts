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
const queryResources = zomeFunction('observation', 'economic_resource', 'query_resources')

export const conformingResources = async (record: ResourceSpecification): Promise<[EconomicResource]> => {
  return (await queryResources({ params: { conformsTo: record.id } })).map(({ economicResource }) => economicResource )
}

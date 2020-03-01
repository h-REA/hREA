/**
 * Resolvers for ResourceSpecification fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'

import {
  Maybe,
  EconomicResource,
  ResourceSpecification,
  Unit,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const queryResources = zomeFunction('observation', 'economic_resource', 'query_resources')
const readUnit = zomeFunction('specification', 'unit', 'get_unit')

export const conformingResources = async (record: ResourceSpecification): Promise<EconomicResource[]> => {
  return (await queryResources({ params: { conformsTo: record.id } })).map(({ economicResource }) => economicResource )
}

export const defaultUnitOfEffort = async (record: ResourceSpecification): Promise<Maybe<Unit>> => {
  if (!record.defaultUnitOfEffort) {
    return null
  }
  return (await readUnit({ id: record.defaultUnitOfEffort })).unit
}

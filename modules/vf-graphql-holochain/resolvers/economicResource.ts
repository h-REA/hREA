/**
 * Resolvers for EconomicResource fields
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { zomeFunction } from '../connection'

import {
  EconomicResource,
  ResourceSpecification,
  Unit,
  ProcessSpecification,
  Action,
  Maybe,
} from '@valueflows/vf-graphql'

const readResources = zomeFunction('observation', 'economic_resource', 'query_resources')
const readUnit = zomeFunction('specification', 'unit', 'get_unit')
const readProcessSpecification = zomeFunction('specification', 'process_specification', 'get_process_specification')
const readAction = zomeFunction('specification', 'action', 'get_action')
const readResourceSpecification = zomeFunction('specification', 'resource_specification', 'get_resource_specification')

export const containedIn = async (record: EconomicResource): Promise<EconomicResource> => {
  return (await readResources({ params: { contains: record.id } })).pop()['economicResource']
}

export const contains = async (record: EconomicResource): Promise<EconomicResource[]> => {
  return (await readResources({ params: { containedIn: record.id } })).map(({ economicResource }) => economicResource)
}

export const conformsTo = async (record: EconomicResource): Promise<ResourceSpecification> => {
  return (await readResourceSpecification({ address: record.conformsTo})).resourceSpecification
}

export const unitOfEffort = async (record: EconomicResource): Promise<Maybe<Unit>> => {
  if (!record.unitOfEffort) {
    return null
  }
  return (await readUnit({ id: record.unitOfEffort })).unit
}

export const stage = async (record: EconomicResource): Promise<ProcessSpecification> => {
  return (await readProcessSpecification({ address: record.stage })).processSpecification
}

export const state = async (record: EconomicResource): Promise<Action> => {
  return (await readAction({ address: record.state }))
}

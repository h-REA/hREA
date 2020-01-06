/**
 * Resolvers for EconomicResource fields
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { zomeFunction } from '../connection'

// 1. Import the GraphQL shema for ResourceSpecification
import {
  EconomicResource,
  ResourceSpecification,
  Unit,
  ProcessSpecification,
  Action
} from '@valueflows/vf-graphql'

const readResources = zomeFunction('observation', 'economic_resource', 'query_resources')
const readUnit = zomeFunction('specification', 'unit', 'get_unit')
const readProcessSpecification = zomeFunction('specification', 'process_specification', 'get_process_specification')
const readAction = zomeFunction('specification', 'action', 'get_action')

//2. make reference to zome function that reads the ResourceSpecification
const readResourceSpecification = zomeFunction('specification', 'resource_specification', 'get_resource_specification')

export const containedIn = async (record: EconomicResource): Promise<EconomicResource> => {
  return (await readResources({ params: { contains: record.id } })).pop()['economicResource']
}

export const contains = async (record: EconomicResource): Promise<[EconomicResource]> => {
  return (await readResources({ params: { containedIn: record.id } })).map(({ economicResource }) => economicResource)
}

//3. export resolver function
export const conformsTo = async (record: EconomicResource): Promise<ResourceSpecification> => {
  return (await readResourceSpecification({ address: record.conformsTo})).resourceSpecification
}

export const unitOfEffort = async(record: EconomicResource): Promise<Unit> => {
  return (await readUnit({ address: record.unitOfEffort })).unit
}

export const stage = async(record: EconomicResource): Promise<ProcessSpecification> => {
  return (await readProcessSpecification({ address: record.stage })).processSpecification
}

export const state = async (record: EconomicResource): Promise<Action> => {
  return (await readAction({ address: record.state}))
}

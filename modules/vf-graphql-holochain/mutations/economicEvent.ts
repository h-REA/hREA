/**
 * Economic event mutations
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  EconomicEventCreateParams,
  EconomicResourceCreateParams,
  EconomicEventUpdateParams,
  EconomicEventResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  event: EconomicEventCreateParams,
  newInventoriedResource?: EconomicResourceCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<EconomicEventResponse>

export interface UpdateArgs {
  event: EconomicEventUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<EconomicEventResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event', 'create_economic_event')
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event', 'update_economic_event')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event', 'delete_economic_event')

  const createEconomicEvent: createHandler = async (root, args) => {
    return runCreate({ event: args.event, new_inventoried_resource: args.newInventoriedResource })
  }

  const updateEconomicEvent: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteEconomicEvent: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createEconomicEvent,
    updateEconomicEvent,
    deleteEconomicEvent,
  }
}

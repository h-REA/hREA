/**
 * Economic event mutations
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { zomeFunction } from '../connection'
import {
  EconomicEventCreateParams,
  EconomicResourceCreateParams,
  EconomicEventUpdateParams,
  EconomicEventResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createEvent = zomeFunction('observation', 'economic_event', 'create_event')
const updateEvent = zomeFunction('observation', 'economic_event', 'update_event')
const deleteEvent = zomeFunction('observation', 'economic_event', 'delete_event')

// CREATE
interface CreateArgs {
  event: EconomicEventCreateParams,
  newInventoriedResource?: EconomicResourceCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<EconomicEventResponse>

export const createEconomicEvent: createHandler = async (root, args) => {
  const { event, newInventoriedResource } = args
  return createEvent({ event, new_inventoried_resource: newInventoriedResource })
}

// UPDATE
interface UpdateArgs {
  event: EconomicEventUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<EconomicEventResponse>

export const updateEconomicEvent: updateHandler = async (root, args) => {
  const { event } = args
  return updateEvent({ event })
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteEconomicEvent: deleteHandler = async (root, args) => {
  return deleteEvent({ address: args.id })
}

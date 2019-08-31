/**
 * Satisfaction mutations
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'
import {
  SatisfactionCreateParams,
  SatisfactionUpdateParams,
  SatisfactionResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('a1_planning', 'satisfaction', 'create_satisfaction')
const updateHandler = zomeFunction('a1_planning', 'satisfaction', 'update_satisfaction')
const deleteHandler = zomeFunction('a1_planning', 'satisfaction', 'delete_satisfaction')

// CREATE
interface CreateArgs {
  satisfaction: SatisfactionCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<SatisfactionResponse>

export const createSatisfaction: createHandler = async (root, args) => {
  return (await createHandler)(args)
}

// UPDATE
interface UpdateArgs {
  satisfaction: SatisfactionUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<SatisfactionResponse>

export const updateSatisfaction: updateHandler = async (root, args) => {
  return (await updateHandler)(args)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteSatisfaction: deleteHandler = async (root, args) => {
  return (await deleteHandler)({ address: args.id })
}

/**
 * Action queries
 *
 * @package: HoloREA
 * @since:   2019-12-23
 */

import { zomeFunction } from '../connection'

import {
  Action,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const read = zomeFunction('specification', 'action', 'get_action')
const readAll = zomeFunction('specification', 'action', 'get_all_actions')

export const action = async (root, args): Promise<Action> => {
  return read(args)
}

export const actions = async (root, args): Promise<Action> => {
  return readAll({})
}

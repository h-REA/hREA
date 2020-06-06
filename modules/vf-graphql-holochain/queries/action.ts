/**
 * Action queries
 *
 * @package: HoloREA
 * @since:   2019-12-23
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Action,
} from '@valueflows/vf-graphql'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const read = mapZomeFn(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readAll = mapZomeFn(dnaConfig, conductorUri, 'specification', 'action', 'get_all_actions')

  return {
    action: async (root, args): Promise<Action> => {
      return read(args)
    },

    actions: async (root, args): Promise<Action> => {
      return readAll({})
    },
  }
}

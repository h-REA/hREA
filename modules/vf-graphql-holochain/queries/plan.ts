/**
 * Top-level queries relating to Plan
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { DNAIdMappings, injectTypename } from '../types'
import { mapZomeFn } from '../connection'

import {
    Plan,
    PlanConnection,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'planning', 'plan', 'get_plan')
  const readAll = mapZomeFn(dnaConfig, conductorUri, 'planning', 'plan_index', 'read_all_plans')

  return {
    plan: injectTypename('Plan', async (root, args): Promise<Plan> => {
      return (await readOne({ address: args.id })).plan
    }),

    plans: async (root, args): Promise<PlanConnection> => {
      return await readAll(null)
    },
  }
}


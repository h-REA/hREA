/**
 * Top-level queries relating to Plan
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { DNAIdMappings, injectTypename, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
    Plan,
    PlanConnection,
    PlanResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, PlanResponse>(dnaConfig, conductorUri, 'plan', 'plan', 'get_plan')
  // const readAll = mapZomeFn(dnaConfig, conductorUri, 'plan', 'plan_index', 'get_all_plans')

  return {
    plan: injectTypename('Plan', async (root, args): Promise<Plan> => {
      return (await readOne({ address: args.id })).plan
    }),

    // plans: async (root, args): Promise<PlanConnection> => {
    //   return await readAll(null)
    // },
  }
}


/**
 * Economic resource mutations
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  EconomicResourceUpdateParams,
  EconomicResourceResponse,
} from '@valueflows/vf-graphql'

export interface UpdateArgs {
  resource: EconomicResourceUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<EconomicResourceResponse>

export default (dnaConfig: DNAIdMappings, conductorUri?: string) => {
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_resource', 'update_resource')

  const updateEconomicResource: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  return {
    updateEconomicResource,
  }
}

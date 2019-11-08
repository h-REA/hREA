/**
 * Economic resource mutations
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { zomeFunction } from '../connection'
import {
  EconomicResourceUpdateParams,
  EconomicResourceResponse,
} from '@valueflows/vf-graphql'

const updateRecord = zomeFunction('observation', 'economic_resource', 'update_resource')

// UPDATE
interface UpdateArgs {
  resource: EconomicResourceUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<EconomicResourceResponse>

export const updateEconomicResource: updateHandler = async (root, args) => {
  return updateRecord(args)
}

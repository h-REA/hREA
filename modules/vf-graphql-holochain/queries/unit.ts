/**
 * Top-level queries relating to Unit
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'

import {
  Unit,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readUnit = zomeFunction('specification', 'unit', 'get_unit')

// Read a single record by ID
export const unit = async (root, args): Promise<Unit> => {
  return (await readUnit({ address: args.id })).unit
}

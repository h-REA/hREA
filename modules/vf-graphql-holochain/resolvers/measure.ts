/**
 * Resolver callbacks for measurement struct sub-fields
 *
 * @package: HoloREA
 * @since:   2019-12-24
 */

import { zomeFunction } from '../connection'

import {
  Measure,
  Unit,
} from '@valueflows/vf-graphql'

const readUnit = zomeFunction('specification', 'unit', 'get_unit')

export const hasUnit = async (record: Measure): Promise<Unit> => {
  return (await readUnit({ id: record.hasUnit })).unit
}

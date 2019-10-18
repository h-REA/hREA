/**
 * Intent record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'

import {
  Intent,
  Satisfaction,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readSatisfactions = zomeFunction('planning', 'satisfaction', 'query_satisfactions')

export const satisfiedBy = async (record: Intent): Promise<[Satisfaction]> => {
  return (await (await readSatisfactions)({ satisfies: record.id })).map(({ satisfaction }) => satisfaction)
}

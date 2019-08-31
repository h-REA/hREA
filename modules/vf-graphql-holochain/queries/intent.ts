/**
 * Top-level Intent queries
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'

import {
  Intent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readRecord = zomeFunction('a1_planning', 'intent', 'get_intent')

// Read a single commitment by ID
export const intent = async (root, args): Promise<Intent> => {
  const { id } = args
  return (await (await readRecord)({ address: id })).intent
}

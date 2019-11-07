/**
 * Top-level queries related to Satisfactions
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'

import {
  Satisfaction,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readRecord = zomeFunction('planning', 'satisfaction', 'get_satisfaction')

// Read a single commitment by ID
export const satisfaction = async (root, args): Promise<Satisfaction> => {
  return (await readRecord({ address: args.id })).satisfaction
}

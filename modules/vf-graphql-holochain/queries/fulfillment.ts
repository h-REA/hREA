/**
 * Top-level queries relating to Fulfillments
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { zomeFunction } from '../connection'

// :TODO: how to inject DNA identifier?
const readFulfullment = zomeFunction('a1_planning', 'fulfillment', 'get_fulfillment')

// Read a single commitment by ID
export const fulfillment = async (root, args) => {
  const { id } = args
  return (await (await readFulfullment)({ address: id })).fulfillment
}

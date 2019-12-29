/**
 * Top-level queries relating to ResourceSpecification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'

import {
  ResourceSpecification,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readResourceSpecification = zomeFunction('specification', 'resource_specification', 'get_resource_specification')

// Read a single record by ID
export const resourceSpecification = async (root, args): Promise<ResourceSpecification> => {
  return (await readResourceSpecification({ address: args.id })).resourceSpecification
}

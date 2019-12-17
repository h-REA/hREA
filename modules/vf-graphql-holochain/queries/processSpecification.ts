/**
 * Top-level queries relating to ProcessSpecification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'

import {
  ProcessSpecification,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readProcessSpecification = zomeFunction('specification', 'process_specification', 'get_process_specification')

// Read a single record by ID
export const processSpecification = async (root, args): Promise<ProcessSpecification> => {
  return (await readProcessSpecification({ address: args.id })).processSpecification
}

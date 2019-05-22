/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { zomeFunction } from '../connection'
import { AnyType, URL, DateTime } from '../types'

const readEvent = zomeFunction('a1_observation', 'main', 'get_event')

export const resolvers = {
  Query: {
    async economicEvent (root, args) {
      const { id } = args
      await (await readEvent)({ address: id })
    }
  },
  Mutation: {

  },
  // :TODO: figure out how scalar types are supposed to be integrated
  AnyType,
  URL,
  DateTime
}

/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import * as Query from '../queries'
import * as Mutation from '../mutations'

import * as EconomicEvent from './economicEvent'
import * as Fulfillment from './fulfillment'
import * as Commitment from './commitment'

// scalar type resolvers
export { AnyType, URI, DateTime } from '../types'
// root schemas
export { Query, Mutation }

// object field resolvers
export {
  EconomicEvent,
  Commitment, Fulfillment,
}

/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import * as Query from '../queries'
import * as Mutation from '../mutations'

import * as Process from './process'
import * as EconomicEvent from './economicEvent'

import * as Commitment from './commitment'
import * as Fulfillment from './fulfillment'

import * as Intent from './intent'
import * as Satisfaction from './satisfaction'

// scalar type resolvers
export { AnyType, URI, DateTime } from '../types'
// root schemas
export { Query, Mutation }

// object field resolvers
export {
  Process,
  EconomicEvent,
  Commitment, Fulfillment,
  Intent, Satisfaction,
}

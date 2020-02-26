/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import * as Query from '../queries'
import * as Mutation from '../mutations'

import * as ResourceSpecification from './resourceSpecification'

import * as Process from './process'
import * as EconomicResource from './economicResource'
import * as EconomicEvent from './economicEvent'

import * as Commitment from './commitment'
import * as Fulfillment from './fulfillment'

import * as Intent from './intent'
import * as Satisfaction from './satisfaction'

import * as Proposal from './proposal'

// scalar type resolvers
export { AnyType, URI, DateTime } from '../types'
// root schemas
export { Query, Mutation }

// union type disambiguation
const EventOrCommitment = {
  __resolveType: (obj, ctx, info) => obj.__typename,
}
const Agent = {
  __resolveType: (obj, ctx, info) => obj.__typename,
}

// object field resolvers
export {
  ResourceSpecification,
  Process,
  EconomicResource,
  EconomicEvent,
  Commitment, Fulfillment,
  Intent, Satisfaction,
  EventOrCommitment,
  Proposal,
}

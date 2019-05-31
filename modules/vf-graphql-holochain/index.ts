/**
 * HoloREA GraphQL schema interface
 *
 * A GraphQL schema (suitable for use with `apollo-link-schema`) which defines
 * bindings between the ValueFlows protocol and a Holochain backend.
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { makeExecutableSchema } from 'graphql-tools'
import { all_vf as typeDefs } from '@valueflows/vf-graphql/typeDefs'

import * as resolvers from './resolvers'

// workaround injected properties added by Typescript compiler, which freaks GraphQL out
if (resolvers['__esModule']) {
  delete resolvers['__esModule']
}

export default makeExecutableSchema({
  typeDefs,
  resolvers,
})

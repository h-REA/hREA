/**
 * HoloREA GraphQL schema interface
 *
 * A GraphQL schema (suitable for use with `apollo-link-schema`) which defines
 * bindings between the ValueFlows protocol and a Holochain backend.
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { makeExecutableSchema } from '@graphql-tools/schema'

import { APIOptions, ResolverOptions, DEFAULT_VF_MODULES } from './types'
import generateResolvers from './resolvers'
import { mapZomeFn } from './connection'
const { buildSchema, printSchema } = require('@valueflows/vf-graphql')

export {
  // direct access to resolver callbacks generator for apps that need to bind to other GraphQL schemas
  generateResolvers,
  // direct access to Holochain zome method bindings for authoring own custom resolvers bound to non-REA DNAs
  mapZomeFn,
}

/**
 * A schema generator ready to be plugged in to a GraphQL client
 *
 * @return GraphQLSchema
 */
export default (options?: APIOptions) => {
  const {
    // use full supported set of modules by default
    enabledVFModules = DEFAULT_VF_MODULES,
    extensionSchemas = [],
    extensionResolvers = {},
  } = (options || {})

  const coreResolvers = generateResolvers(options as ResolverOptions)
  const resolvers = {
    ...coreResolvers,
    ...extensionResolvers,
    Query: {
      ...(coreResolvers.Query || {}),
      ...(extensionResolvers.Query || {})
    },
    Mutation: {
      ...(coreResolvers.Mutation || {}),
      ...(extensionResolvers.Mutation || {})
    }
  }

  return makeExecutableSchema({
    typeDefs: printSchema(buildSchema(enabledVFModules, extensionSchemas)),
    resolvers,
  })
}

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

import { APIOptions, ResolverOptions } from './types'
import generateResolvers, { DEFAULT_VF_MODULES } from './resolvers'
const { buildSchema, printSchema } = require('@valueflows/vf-graphql')

// direct access to resolver callbacks generator for apps that need it
export { generateResolvers }

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

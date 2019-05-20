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

export * from './resolvers'

// :TODO: setup a build system so this module can export the fully flattened schema
// import { loader } from 'graphql.macro'
// const structs = loader('@valueflows/vf-graphql-schemas/structs.gql')
// const agent = loader('@valueflows/vf-graphql-schemas/agent.gql')
// const observation = loader('@valueflows/vf-graphql-schemas/observation.gql')
// const planning = loader('@valueflows/vf-graphql-schemas/planning.gql')
// const knowledge = loader('@valueflows/vf-graphql-schemas/knowledge.gql')
// const query = loader('@valueflows/vf-graphql-schemas/query.gql')
// const mutation = loader('@valueflows/vf-graphql-schemas/mutation.gql')

// export default makeExecutableSchema({
//   typeDefs: [structs, agent, observation, planning, knowledge, query, mutation],
//   resolvers
// })

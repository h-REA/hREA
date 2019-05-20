/**
 * :TODO: deprecate this, use pre-bundled schema in vf-graphql-holochain
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { makeExecutableSchema } from 'graphql-tools'

import { resolvers } from '@valueflows/vf-graphql-holochain'

import { loader } from 'raw.macro'
const structs = loader('@valueflows/vf-graphql-schemas/structs.gql')
const agent = loader('@valueflows/vf-graphql-schemas/agent.gql')
const observation = loader('@valueflows/vf-graphql-schemas/observation.gql')
const planning = loader('@valueflows/vf-graphql-schemas/planning.gql')
const knowledge = loader('@valueflows/vf-graphql-schemas/knowledge.gql')
const query = loader('@valueflows/vf-graphql-schemas/query.gql')
const mutation = loader('@valueflows/vf-graphql-schemas/mutation.gql')

export default makeExecutableSchema({
  typeDefs: [structs, agent, observation, planning, knowledge, query, mutation],
  resolvers
})

import { setClient, query, mutation, subscribe } from "svelte-apollo";
import { gql } from 'graphql-tag'

const AGENT_CORE_FIELDS = gql`
    fragment AgentCoreFields on Organization {
      id
      revisionId
      name
      image
      note
      classifiedAs
    }
  `

export const GET_ALL_AGENTS = gql`
  ${AGENT_CORE_FIELDS}
  query {
    agents(last: 100000) {
      edges {
        cursor
        node {
          ...AgentCoreFields
        }
      }
    }
  }
  `
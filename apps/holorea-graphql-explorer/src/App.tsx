import React, { Component } from 'react'
import { GraphQLSchema, parse, DocumentNode } from 'graphql'
import { SchemaLink } from '@apollo/client/link/schema'
// @ts-ignore
import GraphiQL, { Fetcher } from 'graphiql'
// @ts-ignore
import GraphiQLExplorer from 'graphiql-explorer'

import bindSchema, { autoConnect, VfModule } from '@valueflows/vf-graphql-holochain'

import 'graphiql/graphiql.css'
import './App.css'

/*
  These defaults, by running them one-by-one from top to bottom
  are intended to help the user familiarize, and overcome the
  basic syntactical issues people can have when new to graphql
  and the specific data formats of hREA.
*/
const DEFAULT_QUERIES = `
mutation CreatePerson {
  createPerson(person: { name: "place your name here" }) {
    agent {
      id
      name
    }
  }
}

mutation AssociateMyAgent {
  associateMyAgent(agentId: "place agent.id here")
}

mutation CreateEconomicEvent {
  createEconomicEvent(
    event: {
      action: "raise",
      provider: "place agent.id",
      receiver: "place agent.id here also",
      resourceClassifiedAs: "https://fish",
      resourceQuantity: {hasNumericalValue: 1},
      hasPointInTime: "2022-06-09T18:51:57.105Z"}
  ) {
    economicEvent {
      action {
        id
      }
      id
      resourceClassifiedAs
      resourceQuantity {
        hasNumericalValue
      }
      hasPointInTime
    }
  }
}

query GetMyAGent {
  myAgent {
    id
    name
    economicEventsAsProvider {
      edges {
        node {
          id
          resourceQuantity {
            hasNumericalValue
          }
          resourceClassifiedAs
          action {
            label
          }
        }
      }
    }
  }
}
`

interface Props {}

interface State {
  schema?: GraphQLSchema,
  link?: SchemaLink,
  fetcher?: Fetcher,
  query?: string,
  explorerIsOpen: boolean,
}

class App extends Component<Props, State> {
  _graphiql?: GraphiQL
  state = {
    schema: undefined,
    link: undefined,
    fetcher: undefined,
    query: DEFAULT_QUERIES,
    explorerIsOpen: false
  }

  constructor(props: Props) {
    super(props)
    this.connect()
  }

  async connect () {
    let { dnaConfig, conductorUri } = await autoConnect()
    const schema = await bindSchema({ dnaConfig, conductorUri })
    const link = new SchemaLink({ schema })

    this.setState({
      schema,
      link,
      fetcher: ((operation: any) => {
        operation.query = parse(operation.query)
        return link.request(operation)
      }) as Fetcher
    })
  }

  _handleEditQuery = (query?: string, documentAST?: DocumentNode): void => this.setState({ query })

  _handleToggleExplorer = () => {
    this.setState({ explorerIsOpen: !this.state.explorerIsOpen })
  }

  render () {
    const { query, schema, fetcher } = this.state

    const nodes = [(
        <GraphiQLExplorer key="explorer"
          schema={schema}
          query={query}
          onEdit={this._handleEditQuery}
          explorerIsOpen={this.state.explorerIsOpen}
          onToggleExplorer={this._handleToggleExplorer}
        />
      ), (fetcher ? (
        <GraphiQL key="giql-main"
          ref={
            //@ts-ignore
            ref => (this._graphiql = ref)
          }
          fetcher={fetcher}
          schema={schema}
          query={query}
          onEditQuery={this._handleEditQuery}>
          <GraphiQL.Toolbar>
            <GraphiQL.Button
              onClick={() => { if (this._graphiql) this._graphiql.handlePrettifyQuery() }}
              label='Prettify'
              title='Prettify Query (Shift-Ctrl-P)'
            />
            <GraphiQL.Button
              onClick={() => { if (this._graphiql) this._graphiql.handleToggleHistory() }}
              label='History'
              title='Show History'
            />
            <GraphiQL.Button
              onClick={this._handleToggleExplorer}
              label='Explorer'
              title='Toggle Explorer'
            />
          </GraphiQL.Toolbar>
        </GraphiQL>
      ) : null)
    ]

    return (
      <div className='graphiql-container'>{nodes}</div>
    )
  }
}

export default App

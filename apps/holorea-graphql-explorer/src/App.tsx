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

# Welcome to the hREA developer console!
#
# If you're new here, there are a few things to do to get you set up.
#
# These queries will run you through a few steps to setup a new
# REA Agent profile and log a test 'economic event' to setup an
# inventory for yourself.

# Step 1: create a new profile for yourself.
# You can edit it later with \`updatePerson\`.

mutation CreatePerson {
  createPerson(person: { name: "place your name here" }) {
    agent {
      id
      name
    }
  }
}

# Step 2: associate the profile with your current access token.
# Use the \`id\` returned from the previous query.

mutation AssociateMyAgent {
  associateMyAgent(agentId: "place agent.id here")
}

# Step 3: log a new \`EconomicEvent\` to start tracking some inventory.
# Use the same Agent ID above for both \`provider\` and \`receiver\`,
# incidating that you initialised the inventory yourself.
# With the given parameters you will end up with
# an inventory for "apples" (defined in http://www.productontology.org/doc/Apple)
# that accounts for a total of 1 apple.

mutation CreateEconomicEvent {
  createEconomicEvent(
    event: {
      action: "raise"
      provider: "place agent.id"
      receiver: "place agent.id here also"
      resourceClassifiedAs: "pto:Apple"
      resourceQuantity: { hasNumericalValue: 1 }
      hasPointInTime: "2022-06-09T18:51:57.105Z"
    }
    newInventoriedResource: {
      name: "My apples"
    }
  ) {
    economicEvent {
      id
      action {
        id
      }
      resourceClassifiedAs
      resourceQuantity {
        hasNumericalValue
      }
      hasPointInTime
    }
    economicResource {
      id
      name
      classifiedAs
      accountingQuantity {
        hasNumericalValue
      }
      onhandQuantity {
        hasNumericalValue
      }
    }
  }
}

# Now that you have created all these records, you should be able to
# retrieve them

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
    # TODO: include inventoried EconomicResources
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

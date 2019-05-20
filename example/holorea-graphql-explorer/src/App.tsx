import React, { Component } from 'react'
import { GraphQLSchema, parse } from 'graphql'
import { execute } from 'apollo-link'
import { SchemaLink } from 'apollo-link-schema'
// @ts-ignore
import GraphiQL from 'graphiql'
// @ts-ignore
import GraphiQLExplorer from 'graphiql-explorer'

import schema from './schema' // :TODO: use @valueflows/vf-graphql-holochain

import 'graphiql/graphiql.css'
import './App.css'

import { getDefaultScalarArgValue, makeDefaultArg } from './CustomArgs'

const DEFAULT_QUERY = `{
  myAgent {
    id
    name
  }
}`

interface State {
  schema?: GraphQLSchema,
  query: string,
  explorerIsOpen: boolean,
}

const link = new SchemaLink({ schema })

// @ts-ignore
const fetcher = (operation) => {
  operation.query = parse(operation.query)
  return execute(link, operation)
}

class App extends Component<{}, State> {
  _graphiql: GraphiQL
  state = {
    schema,
    query: DEFAULT_QUERY,
    explorerIsOpen: false
  }

  _handleEditQuery = (query: string): void => this.setState({ query })

  _handleToggleExplorer = () => {
    this.setState({ explorerIsOpen: !this.state.explorerIsOpen })
  }

  render () {
    const { query, schema } = this.state
    return (
      <div className='graphiql-container'>
        <GraphiQLExplorer
          schema={schema}
          query={query}
          onEdit={this._handleEditQuery}
          explorerIsOpen={this.state.explorerIsOpen}
          onToggleExplorer={this._handleToggleExplorer}
          getDefaultScalarArgValue={getDefaultScalarArgValue}
          makeDefaultArg={makeDefaultArg}
        />
        <GraphiQL
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
              onClick={() => this._graphiql.handlePrettifyQuery()}
              label='Prettify'
              title='Prettify Query (Shift-Ctrl-P)'
            />
            <GraphiQL.Button
              onClick={() => this._graphiql.handleToggleHistory()}
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
      </div>
    )
  }
}

export default App

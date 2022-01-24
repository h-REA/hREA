import React, { Component } from 'react'
import { GraphQLSchema, parse, DocumentNode } from 'graphql'
import { SchemaLink } from '@apollo/client/link/schema'
// @ts-ignore
import GraphiQL, { Fetcher } from 'graphiql'
// @ts-ignore
import GraphiQLExplorer from 'graphiql-explorer'

import bindSchema, { openConnection, DNAMappings, CellId } from '@valueflows/vf-graphql-holochain'

import 'graphiql/graphiql.css'
import './App.css'

const DEFAULT_QUERY = `{
  myAgent {
    id
    name
  }
}`

interface Props {}

interface State {
  schema?: GraphQLSchema,
  link?: SchemaLink,
  fetcher?: Fetcher,
  query?: string,
  explorerIsOpen: boolean,
}

type ActualInstalledCell = {  // :TODO: remove this when fixed in tryorama
    cell_id: CellId;
    role_id: string;
}

class App extends Component<Props, State> {
  _graphiql?: GraphiQL
  state = {
    schema: undefined,
    link: undefined,
    fetcher: undefined,
    query: DEFAULT_QUERY,
    explorerIsOpen: false
  }

  constructor(props: Props) {
    super(props)
    this.connect()
  }

  async connect () {
    let dnaMappings: DNAMappings

    const conn = await openConnection(process.env.REACT_APP_HC_CONN_URL as string);
    const appInfo = await conn.appInfo({ installed_app_id: (process.env.REACT_APP_HC_APP_ID as string) })
    if (!appInfo) {
      throw new Error(`appInfo call failed for Holochain app '${process.env.REACT_APP_HC_APP_ID}' - ensure the name is correct and that the agent's app installation has not failed`)
    }

    dnaMappings = (appInfo['cell_data'] as unknown[] as ActualInstalledCell[]).reduce((mappings, { cell_id, role_id }) => {
      const hrea_cell_match = role_id.match(/hrea_(\w+)_\d+/)
      if (!hrea_cell_match) { return mappings }

      mappings[hrea_cell_match[1] as keyof DNAMappings] = cell_id
      return mappings
    }, {} as DNAMappings)
    console.log('Connecting to detected Holochain cells:', dnaMappings)

    const schema = await bindSchema({
      dnaConfig: dnaMappings,
      conductorUri: process.env.REACT_APP_HC_CONN_URL as string,
    })
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

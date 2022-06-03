/**
 * base types for GraphQL query layer
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { AppSignalCb, CellId, HeaderHash } from '@holochain/client'
import { IResolvers } from '@graphql-tools/utils'
import { GraphQLScalarType } from 'graphql'
import { Kind } from 'graphql/language'

// Configuration object to allow specifying custom conductor DNA IDs to bind to.
// Default is to use a DNA with the same ID as the mapping ID (ie. agent = "agent")
export interface DNAIdMappings {
  agent?: CellId,
  agreement?: CellId,
  observation?: CellId,
  planning?: CellId,
  proposal?: CellId,
  specification?: CellId,
}

export { CellId }

// Options for resolver generator
export interface ResolverOptions {
  // Array of ValueFlows module names to include in the schema
  // @see https://lab.allmende.io/valueflows/vf-schemas/vf-graphql#generating-schemas
  enabledVFModules?: VfModule[],

  // Mapping of DNA identifiers to runtime `CellId`s to bind to.
  dnaConfig: DNAIdMappings,

  // Custom Holochain conductor URI to use with this instance, to support connecting to multiple conductors.
  // If not specified, connects to the local conductor or the URI stored in `process.env.REACT_APP_HC_CONN_URL`.
  conductorUri: string,

  // Callback to listen for signals from the Holochain app websocket, to support realtime event notifications.
  traceAppSignals?: AppSignalCb,
}

// Schema generation options to be passed to vf-graphql
// @see https://github.com/valueflows/vf-graphql/#generating-schemas
export interface ExtensionOptions {
  // Array of additional SDL schema strings to bundle into the resultant schema in addition to the
  // specified modular sub-section of the ValueFlows spec.
  extensionSchemas?: string[],

  // Additional resolver callbacks to inject into the schema in addition to the specified bound
  // set of Holo-REA DNA resolvers.
  // Used for injecting implementation logic for `extensionSchemas`: not recommended for overriding
  // parts of the Holo-REA core behaviour!
  extensionResolvers?: IResolvers,
}

// types that serialize for rust zome calls
// start of section
export interface ReadParams {
  address: AddressableIdentifier,
}
export interface ById {
  id: AddressableIdentifier,
}

export type AddressableIdentifier = string
export type CommitmentAddress = AddressableIdentifier
export type ProcessAddress = AddressableIdentifier
export type FulfillmentAddress = AddressableIdentifier
export type SatisfactionAddress = AddressableIdentifier
export type AgreementAddress = AddressableIdentifier
export type PlanAddress = AddressableIdentifier
export type ProposalAddress = AddressableIdentifier
export type IntentAddress = AddressableIdentifier
export type AgentAddress = AddressableIdentifier
export type EconomicResourceAddress = AddressableIdentifier
export type EconomicEventAddress = AddressableIdentifier
export type ResourceSpecificationAddress = AddressableIdentifier
export type ProposedIntentAddress = AddressableIdentifier

export interface ByRevision {
  revisionId: string
}
// end of section

export type APIOptions = ResolverOptions & ExtensionOptions

// helpers for resolvers to inject __typename parameter for union type disambiguation
// ...this might be unnecessarily present due to lack of familiarity with GraphQL?

type ObjDecorator<T> = (obj: T) => T
type Resolver<T> = (root, args) => Promise<T>

export function addTypename<T> (name: string): ObjDecorator<T> {
  return (obj) => {
    obj['__typename'] = name
    return obj
  }
}

export function injectTypename<T> (name: string, fn: Resolver<T>): Resolver<T> {
  return async (root, args): Promise<T> => {
    const data = await fn(root, args)
    data['__typename'] = name
    return data
  }
}

// enum containing all the possible VF modules, including
// the ones that haven't been implemented within holo-rea yet
// -> https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/tree/sprout/lib/schemas
export enum VfModule {
  Agent = 'agent',
  Agreement = 'agreement',
  Appreciation = 'appreciation',
  Claim = 'claim',
  Geolocation = 'geolocation',
  History = 'history',
  Knowledge = 'knowledge',
  Measurement = 'measurement',
  Observation = 'observation',
  Plan = 'plan',
  Planning = 'planning',
  Proposal = 'proposal',
  Recipe = 'recipe',
  Scenario = 'scenario',
}

// default 'full suite' VF module set supported by Holo-REA

export const DEFAULT_VF_MODULES = [
  VfModule.Agent,
  VfModule.Agreement,
  VfModule.Knowledge,
  VfModule.Measurement,
  VfModule.Observation,
  VfModule.Planning,
  VfModule.Proposal,
  VfModule.Plan,
]

// scalar types

export const URI = new GraphQLScalarType({
  name: 'URI',
  description: 'The `URI` type declares a reference to any resolvable resource.',
  serialize: (v) => v,
  parseValue: (v) => v,
  parseLiteral (ast) {
    if (ast.kind === Kind.STRING) {
      if (!ast.value.match(/^\w+:/)) {
        throw new Error('Unable to parse URI- invalid format')
      }
      return ast.value
    }
    return null
  },
})

/**
 * base types for GraphQL query layer
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { AppSignalCb, CellId } from '@holochain/client'
import { IResolvers } from '@graphql-tools/utils'
import { GraphQLScalarType } from 'graphql'
import { Kind, ValueNode, VariableNode } from 'graphql/language/index.js'
import Big from 'big.js'

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
  // Array of ValueFlows modules to include in the schema
  // @see https://lab.allmende.io/valueflows/vf-schemas/vf-graphql#generating-schemas
  enabledVFModules: VfModule[],

  // Mapping of DNA identifiers to runtime `CellId`s to bind to.
  dnaConfig: DNAIdMappings,

  // Custom Holochain conductor URI to use with this instance, to support connecting to multiple conductors.
  // If not specified, connects to the local conductor or the URI stored in `process.env.REACT_APP_HC_CONN_URL`.
  conductorUri: string,

  // Custom Admin Holochain conductor URI to use with this instance, to support connecting to multiple conductors.
  // If not specified, connects to the local conductor or the URI stored in `process.env.REACT_APP_HC_ADMIN_CONN_URL`.
  adminConductorUri: string,

  // This is needed in order to fetch the installed CellId's and grant authorization
  // by the Admin API for any zome functions on those Cells to be called
  appId: string,

  // Callback to listen for signals from the Holochain app websocket, to support realtime event notifications.
  traceAppSignals?: AppSignalCb,
}

// Schema generation options to be passed to vf-graphql
// @see https://lab.allmende.io/valueflows/vf-schemas/vf-graphql#generating-schemas
export interface ExtensionOptions {
  // Array of additional SDL schema strings to bundle into the resultant schema in addition to the
  // specified modular sub-section of the ValueFlows spec.
  extensionSchemas?: string[],

  // Additional resolver callbacks to inject into the schema in addition to the specified bound
  // set of hREA DNA resolvers.
  // Used for injecting implementation logic for `extensionSchemas`: not recommended for overriding
  // parts of the hREA core behaviour!
  extensionResolvers?: IResolvers,
}

export type BindSchemaOptions = Pick<ResolverOptions, 'dnaConfig' | 'conductorUri' | 'adminConductorUri' | 'appId' | 'traceAppSignals'>
  & {
    // optional because DEFAULT_VF_MODULES is assigned as fallback
    enabledVFModules?: VfModule[] 
  }
  & ExtensionOptions

// Types that serialize for rust zome calls
// start of section
export interface ReadParams {
  address: AddressableIdentifier,
}
export interface ById {
  id: AddressableIdentifier,
}
export interface ByRevision {
  revisionId: AddressableIdentifier,
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
export type ProcessSpecificationAddress = AddressableIdentifier

export interface ByRevision {
  revisionId: string
}
// end of section

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

// VfModule listing.
// for the reference, see:
// https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/tree/sprout/lib/schemas

// enum containing the not yet implemented VfModule
// obviously, move the variant to VfModule if implementing
// it
enum UnimplementedVfModule {
  Geolocation = 'geolocation',
  History = 'history',
  Recipe = 'recipe',
  Scenario = 'scenario',
  ProductBatch = 'product_batch',
  Appreciation = 'appreciation',
  Claim = 'claim',
  Filtering = 'filtering',
  Ordering = 'ordering',
}

// enum containing the implemented VF modules.
// separate from above so that developers can import the enum and not turn on useless
// features
export enum VfModule {
  // always required regardless
  // see: https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/index.js#L29
  Util = 'util',
  Pagination = 'pagination',
  History = 'history',
  // optional to enable
  Agent = 'agent',
  Agreement = 'agreement',
  Action = 'action',
  ProcessSpecification = 'process_specification',
  ResourceSpecification = 'resource_specification',
  Measurement = 'measurement',
  Observation = 'observation',
  Process = 'process',
  Plan = 'plan',
  Fulfillment = 'fulfillment',
  Intent = 'intent',
  Commitment = 'commitment',
  Satisfaction = 'satisfaction',
  Proposal = 'proposal',
}

// default 'full suite' VF module set supported by hREA
export const DEFAULT_VF_MODULES = [
  VfModule.History,
  // Specification DNA
  VfModule.Action,
  VfModule.ProcessSpecification,
  VfModule.ResourceSpecification,
  VfModule.Measurement,
  // Agent DNA
  VfModule.Agent,
  // Agreement DNA
  VfModule.Agreement,
  // Observation DNA
  VfModule.Observation,
  VfModule.Process,
  // Proposal DNA
  VfModule.Proposal,
  // Plan DNA
  VfModule.Plan,
  // Planning DNA
  VfModule.Fulfillment,
  VfModule.Intent,
  VfModule.Commitment,
  VfModule.Satisfaction,
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

// :TODO: this should be a GraphQLScalarType<Big, string> to avoid precision loss at API boundary
export const Decimal: GraphQLScalarType<Big, number> = new GraphQLScalarType({
  name: 'Decimal',
  description: 'The `Decimal` scalar type to handle precision arithmetic and potentially large values.',
  serialize: (v: unknown) => (v as Big).toNumber(),
  parseValue: (v: unknown) => Big(v as number),
  parseLiteral(ast: ValueNode) {
    if (ast.kind !== Kind.STRING && ast.kind !== Kind.INT && ast.kind !== Kind.FLOAT) {
      // @ts-ignore
      throw new TypeError(String(ast.value) + ' is not a valid decimal value.')
    }

    return Big(ast.value)
  }
})

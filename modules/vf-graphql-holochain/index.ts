import { makeExecutableSchema } from '@graphql-tools/schema';
import { type GraphQLSchema } from 'graphql';
import { generateResolvers } from './src/resolvers/index.js';
// @ts-ignore
import { buildSchema, printSchema } from '@valueflows/vf-graphql'

type hREASchemaParams = {
  appWebSocket: any;
  roleName: string;
  cell?: any; // If you want to pass a cell instead of appWebSocket and roleName
}

export function createHolochainSchema(params: hREASchemaParams): GraphQLSchema {
  const cell = {
    callZome: async function(args) {
      return params.appWebSocket.callZome({
        cap_secret: null,
        role_name: params.roleName,
        zome_name: args.zome_name,
        fn_name: args.fn_name,
        payload: args.payload,
      }, 999999);
    }
  }

  const resolvers = generateResolvers(params.cell || cell)
  const enabledVFModules = [
    'util',
    'pagination',
    'history',
    'agent',
    'action',
    'plan',
    'util',
    'commitment',
    'proposal',
    'recipe',
    'process',
    'measurement',
    'observation',
    'process_specification',
    'resource_specification',
    'agreement',
    'intent',
  ]

  const overriddenExtensionSchemas = []

  // VF 1.0 extension: vf:Proposal.purpose (vf:ProposalPurpose = offer | request).
  // The base @valueflows/vf-graphql schema already declares the `offers` and
  // `requests` queries (resolved in src/queries) but does not carry the
  // `purpose` field on the Proposal type or its input params, so we add it here
  // rather than forking the schema package.
  const vf1ProposalPurposeExtension = `
    enum ProposalPurpose {
      offer
      request
    }
    extend type Proposal {
      "Whether this proposal is published as an offer or a request (vf:Proposal.purpose, VF 1.0)."
      purpose: ProposalPurpose
    }
    extend input ProposalCreateParams {
      purpose: ProposalPurpose
    }
    extend input ProposalUpdateParams {
      purpose: ProposalPurpose
    }

    extend type ResourceSpecification {
      "Whether this specification is used as a medium of exchange (vf:ResourceSpecification.mediumOfExchange, VF 1.0)."
      mediumOfExchange: Boolean
    }
    extend input ResourceSpecificationCreateParams {
      mediumOfExchange: Boolean
    }
    extend input ResourceSpecificationUpdateParams {
      mediumOfExchange: Boolean
    }

    extend type EconomicEvent {
      "Reciprocal counterpart of realizationOf (vf:EconomicEvent.reciprocalRealizationOf, VF 1.0)."
      reciprocalRealizationOf: Agreement
    }
    extend input EconomicEventCreateParams {
      reciprocalRealizationOf: ID
    }
    extend input EconomicEventUpdateParams {
      reciprocalRealizationOf: ID
    }

    extend type Commitment {
      "Reciprocal counterpart of clauseOf (vf:Commitment.reciprocalClauseOf, VF 1.0)."
      reciprocalClauseOf: Agreement
    }
    extend input CommitmentCreateParams {
      reciprocalClauseOf: ID
    }
    extend input CommitmentUpdateParams {
      reciprocalClauseOf: ID
    }

    extend type EconomicEvent {
      "The claim this event settles (vf:EconomicEvent.settles, VF 1.0)."
      settles: Claim
    }
    extend input EconomicEventCreateParams {
      settles: ID
    }
    extend input EconomicEventUpdateParams {
      settles: ID
    }

    "vf:Claim (VF 1.0): a claim for future economic events in reciprocity for an event that already occurred."
    type Claim {
      id: ID!
      revisionId: ID!
      action: Action!
      resourceClassifiedAs: [URI!]
      resourceQuantity: Measure
      effortQuantity: Measure
      triggeredBy: EconomicEvent!
      due: DateTime
      created: DateTime
      finished: Boolean
      note: String
      agreedIn: URI
      "VF 1.0: the EconomicEvents that settle this claim (reverse of EconomicEvent.settles)."
      settledBy: [EconomicEvent!]
    }
    input ClaimCreateParams {
      action: ID!
      resourceClassifiedAs: [URI!]
      resourceQuantity: IMeasure
      effortQuantity: IMeasure
      triggeredBy: ID!
      due: DateTime
      created: DateTime
      finished: Boolean
      note: String
      agreedIn: URI
    }
    input ClaimUpdateParams {
      revisionId: ID!
      action: ID
      resourceClassifiedAs: [URI!]
      resourceQuantity: IMeasure
      effortQuantity: IMeasure
      triggeredBy: ID
      due: DateTime
      created: DateTime
      finished: Boolean
      note: String
      agreedIn: URI
    }
    type ClaimResponse {
      claim: Claim!
    }
    type ClaimEdge {
      node: Claim!
      cursor: String!
    }
    type ClaimConnection {
      edges: [ClaimEdge!]!
      pageInfo: PageInfo!
    }
    extend type Query {
      claim(id: ID!): Claim
      claims(first: Int, after: String, last: Int, before: String): ClaimConnection
    }
    extend type Mutation {
      createClaim(claim: ClaimCreateParams!): ClaimResponse!
      updateClaim(claim: ClaimUpdateParams!): ClaimResponse!
      deleteClaim(revisionId: ID!): Boolean!
    }

    "vf:SpatialThing (VF 1.0): a physical mappable location."
    type SpatialThing {
      id: ID!
      revisionId: ID!
      name: String!
      mappableAddress: String
      lat: Decimal
      long: Decimal
      alt: Decimal
      note: String
    }
    input SpatialThingCreateParams {
      name: String!
      mappableAddress: String
      lat: Decimal
      long: Decimal
      alt: Decimal
      note: String
    }
    input SpatialThingUpdateParams {
      revisionId: ID!
      name: String
      mappableAddress: String
      lat: Decimal
      long: Decimal
      alt: Decimal
      note: String
    }
    type SpatialThingResponse {
      spatialThing: SpatialThing!
    }
    type SpatialThingEdge {
      node: SpatialThing!
      cursor: String!
    }
    type SpatialThingConnection {
      edges: [SpatialThingEdge!]!
      pageInfo: PageInfo!
    }
    extend type Query {
      spatialThing(id: ID!): SpatialThing
      spatialThings(first: Int, after: String, last: Int, before: String): SpatialThingConnection
    }
    extend type Mutation {
      createSpatialThing(spatialThing: SpatialThingCreateParams!): SpatialThingResponse!
      updateSpatialThing(spatialThing: SpatialThingUpdateParams!): SpatialThingResponse!
      deleteSpatialThing(revisionId: ID!): Boolean!
    }

    "vf:AgreementBundle (VF 1.0): a grouping of agreements."
    type AgreementBundle {
      id: ID!
      revisionId: ID!
      name: String
      note: String
      agreements: [Agreement!]
    }
    input AgreementBundleCreateParams {
      name: String
      note: String
      agreements: [ID!]
    }
    input AgreementBundleUpdateParams {
      revisionId: ID!
      name: String
      note: String
      agreements: [ID!]
    }
    type AgreementBundleResponse {
      agreementBundle: AgreementBundle!
    }
    type AgreementBundleEdge {
      node: AgreementBundle!
      cursor: String!
    }
    type AgreementBundleConnection {
      edges: [AgreementBundleEdge!]!
      pageInfo: PageInfo!
    }
    extend type Query {
      agreementBundle(id: ID!): AgreementBundle
      agreementBundles(first: Int, after: String, last: Int, before: String): AgreementBundleConnection
    }
    extend type Mutation {
      createAgreementBundle(agreementBundle: AgreementBundleCreateParams!): AgreementBundleResponse!
      updateAgreementBundle(agreementBundle: AgreementBundleUpdateParams!): AgreementBundleResponse!
      deleteAgreementBundle(revisionId: ID!): Boolean!
    }
  `

  const executableSchema = makeExecutableSchema({
    typeDefs: [
      printSchema(buildSchema(enabledVFModules, overriddenExtensionSchemas)),
      vf1ProposalPurposeExtension,
    ],
    resolvers,
  });

  return executableSchema
}

// Verifies the vf:Proposal.purpose GraphQL surface exists in the built adapter
// schema, without needing a running conductor. Run: node scripts/verify-purpose-schema.mjs
import { createHolochainSchema } from '../modules/vf-graphql-holochain/build/index.js'

const cell = { callZome: async () => null }
const schema = createHolochainSchema({ appWebSocket: null, roleName: 'hrea', cell })

const fieldOf = (typeName, field) => {
  const t = schema.getType(typeName)
  return !!(t && t.getFields && t.getFields()[field])
}
const enumValues = (typeName) => {
  const t = schema.getType(typeName)
  return t && t.getValues ? t.getValues().map((v) => v.name) : null
}

const checks = {
  'Proposal.purpose': fieldOf('Proposal', 'purpose'),
  'ProposalCreateParams.purpose': fieldOf('ProposalCreateParams', 'purpose'),
  'ProposalUpdateParams.purpose': fieldOf('ProposalUpdateParams', 'purpose'),
  'Query.offers': fieldOf('Query', 'offers'),
  'Query.requests': fieldOf('Query', 'requests'),
  'ProposalPurpose enum': JSON.stringify(enumValues('ProposalPurpose')),
  // VF 1.0 ResourceSpecification booleans
  'ResourceSpecification.substitutable': fieldOf('ResourceSpecification', 'substitutable'),
  'ResourceSpecification.mediumOfExchange': fieldOf('ResourceSpecification', 'mediumOfExchange'),
  'ResourceSpecificationCreateParams.mediumOfExchange': fieldOf('ResourceSpecificationCreateParams', 'mediumOfExchange'),
  // VF 1.0 EconomicEvent.effortQuantity
  'EconomicEvent.effortQuantity': fieldOf('EconomicEvent', 'effortQuantity'),
  'EconomicEventCreateParams.effortQuantity': fieldOf('EconomicEventCreateParams', 'effortQuantity'),
  // VF 1.0 reciprocal Agreement fields
  'EconomicEvent.reciprocalRealizationOf': fieldOf('EconomicEvent', 'reciprocalRealizationOf'),
  'EconomicEventCreateParams.reciprocalRealizationOf': fieldOf('EconomicEventCreateParams', 'reciprocalRealizationOf'),
  'Commitment.reciprocalClauseOf': fieldOf('Commitment', 'reciprocalClauseOf'),
  'CommitmentCreateParams.reciprocalClauseOf': fieldOf('CommitmentCreateParams', 'reciprocalClauseOf'),
  // VF 1.0 Claim entry type + settles
  'Claim type exists': !!schema.getType('Claim'),
  'Claim.triggeredBy': fieldOf('Claim', 'triggeredBy'),
  'Claim.action': fieldOf('Claim', 'action'),
  'Query.claims': fieldOf('Query', 'claims'),
  'Mutation.createClaim': fieldOf('Mutation', 'createClaim'),
  'EconomicEvent.settles': fieldOf('EconomicEvent', 'settles'),
  'EconomicEventCreateParams.settles': fieldOf('EconomicEventCreateParams', 'settles'),
  // VF 1.0 SpatialThing entry type
  'SpatialThing type exists': !!schema.getType('SpatialThing'),
  'SpatialThing.lat': fieldOf('SpatialThing', 'lat'),
  'SpatialThing.mappableAddress': fieldOf('SpatialThing', 'mappableAddress'),
  'Query.spatialThings': fieldOf('Query', 'spatialThings'),
  'Mutation.createSpatialThing': fieldOf('Mutation', 'createSpatialThing'),
  // VF 1.0 ProductBatch entry type
  'ProductBatch type exists': !!schema.getType('ProductBatch'),
  'ProductBatch.batchNumber': fieldOf('ProductBatch', 'batchNumber'),
  'Query.productBatches': fieldOf('Query', 'productBatches'),
  'Mutation.createProductBatch': fieldOf('Mutation', 'createProductBatch'),
  // VF 1.0 AgreementBundle entry type
  'AgreementBundle type exists': !!schema.getType('AgreementBundle'),
  'AgreementBundle.agreements': fieldOf('AgreementBundle', 'agreements'),
  'Query.agreementBundles': fieldOf('Query', 'agreementBundles'),
  'Mutation.createAgreementBundle': fieldOf('Mutation', 'createAgreementBundle'),
}

let ok = true
for (const [name, result] of Object.entries(checks)) {
  const pass = result === true || result === '["offer","request"]'
  if (!pass) ok = false
  console.log(`${pass ? 'PASS' : 'FAIL'}  ${name}: ${result}`)
}
console.log(ok ? '\nALL SCHEMA CHECKS PASSED' : '\nSCHEMA CHECKS FAILED')
process.exit(ok ? 0 : 1)

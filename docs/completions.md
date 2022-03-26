# holo-rea Graphql API Completions & Statuses

From the point-of-view of someone calling through Graphql, what is the overall status of each function that exists in the graphql schema. Many functions are not yet implemented so it is important to know that upfront, which are, and which aren't.

A filtered list of related github issues for tracking these work statuses, so that you can contribute, or report or discuss issues, can be found here: https://github.com/holo-rea/holo-rea/labels/graphql-api

## System of Record Comparison

All of the implementation details should be sourced from the [Valueflows RDF Turtle file](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL) (here's a [Formatted View](http://150.146.207.114/lode/extract?owlapi=true&url=https://lab.allmende.io/valueflows/valueflows/-/raw/master/release-doc-in-process/all_vf.TTL)), which is the system of record. While you are looking around, please note that the objects themselves don't have property definitions. The properties themselves define which objects the apply to in the `rdfs:domain` field. The range of values the properties can take is defined by the `rdfs:range` field This is because RDF views these things like arrows or maps, going from the domain to the range.

The top level objects found in the spec are:

**Key**
| symbol              | meaning               |
| ------------------- | --------------------- |
| :grey_exclamation:  | Not used              |
| -                   | Not found/not started |
| :hammer_and_wrench: | In progress           |
| :heavy_check_mark:  | Done                  |
| K                   | Knowledge Layer       |
| P                   | Planning Layer       |
| O                   | Observation Layer       |

**Outside Ontologies**
| RDF Object                                                                              | vf-schema file                                                                                                                           | zome                                                                                                     | comments |
| --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- | -------- |
| [foaf:Agent](http://xmlns.com/foaf/spec/)                                               | :grey_exclamation: [agent](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/agent.gql)                 | :grey_exclamation:                                                                                       | |
| [org:Organization](https://www.w3.org/TR/vocab-org/)                                    | :grey_exclamation:                                                                                                                       | :grey_exclamation:                                                                                       | |
| [om2:Measure](https://raw.githubusercontent.com/HajoRijgersberg/OM/master/om-2.0.rdf)   | [measurement](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/measurement.gql#L64)                    | [lib/vf_measurement](https://github.com/holo-rea/holo-rea/blob/sprout/lib/vf_measurement/src/lib.rs#L19) | |
| [om2:Unit](https://raw.githubusercontent.com/HajoRijgersberg/OM/master/om-2.0.rdf)      | :grey_exclamation: [measurement](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/measurement.gql#L48) | :grey_exclamation: [rea_unit](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_unit)           | This is a technicality. The general shape of it is correct, however the ontology represents a hierarchy of units that are not correctly reflected in the backend since it only stores a label and a symbol. The full ontology allows for more flexibility with prefixes, dimension, exponent, etc.; it has enough information to allow conversion between units. It would be hard to implement without a triple-store. |
| [geo:SpatialThing](https://www.w3.org/2003/01/geo/)                                     | [geolocation](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/geolocation.gql#L15)                    | -                                                                                                        | |
| [time](https://www.w3.org/2006/time#)                                                   | :grey_exclamation:                                                                                                                       | :grey_exclamation:                                                                                       | vf-schema: The GraphQL spec only uses the `DateTime and Duration` scalars. |
| [cd:created](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/#created) | :grey_exclamation:                                                                                                                       | :grey_exclamation:                                                                                       | vf-schema: GraphQL spec only uses the `DateTime` scalar. |
| [skos:note](https://www.w3.org/TR/skos-reference/#note)                                 | :grey_exclamation:                                                                                                                       | :grey_exclamation:                                                                                       | vf-schema: Just a `String`. |
| [dtype:numericUnion](http://www.linkedmodel.org/schema/dtype#numericUnion)              | :grey_exclamation:                                                                                                                       | :grey_exclamation:                                                                                       | This is only needed for the [`om2:hasNumericalValue`](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L549), so it's only internal. | 


You may notice there is no specification of an Agent. This is because the Valueflows RDF spec uses the [FOAF Vocabulary](http://xmlns.com/foaf/spec/) and the [Organization Ontology](https://www.w3.org/TR/vocab-org/). The holo-rea project has [it's own set of concepts right now](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/agent.gql).

| layer | RDF object                                                                                                                                                                | vf-schemas file                                                                                                                      | zome                                                                                                            | hrea "module" or DNA                                                                  | comments                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| K     | [Scenario Definition](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L124)                                                 | [scenario](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/scenario.gql#L44)                      | -                                                                                                               | -                                                                                     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| K     | [Process Specification](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L104)                                               | [knowledge](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/knowledge.gql#L70)                    | [rea_process_specification](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_process_specification)   | [specification](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/specification) |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| K     | [Resource Specification](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L92)                                               | [knowledge](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/knowledge.gql#L45)                    | [rea_resource_specification](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_resource_specification) | [specification](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/specification) | zome: Missing `resource_classified_as`, `default_unit_of_resource`.                                                                                                                                                                                                                                                                                                                                                                                                                               |
| K     | [Action](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L32)                                                               | [knowledge](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/knowledge.gql#L19)                    | [rea_action](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_action/zome)                            | [specification](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/specification) | vf-schema: Missing `containedEffect`, `locationEffect`. zome: Same as vf-schema.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| K     | [Agent Relationship Role](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L74)                                              | [agent](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/agent.gql#L126)                           | -                                                                                                               | -                                                                                     | vf-schema: Missing `roleBehavior`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| K     | [Role Behavior](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L80)                                                        | -                                                                                                                                    | -                                                                                                               | -                                                                                     | vf-schema: This doesn't seem to be implemented yet.                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| K     | [Recipe Exchange](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L118)                                                     | [recipe](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/recipe.gql#L106)                         | -                                                                                                               | -                                                                                     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| K     | [Recipe Flow](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L112)                                                         | [recipe](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/recipe.gql#L53)                          | -                                                                                                               | -                                                                                     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| K     | [Recipe Process](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L98)                                                       | [recipe](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/recipe.gql#L84)                          | -                                                                                                               | -                                                                                     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| K     | [Recipe Resource](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L86)                                                      | [recipe](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/recipe.gql#L18)                          | -                                                                                                               | -                                                                                     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| P     | [Scenario](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L86)                                                             | [scenario](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/scenario.gql#L16)                      | -                                                                                                               | -                                                                                     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| P     | [Plan](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L133)                                                                | [plan](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/plan.gql#L16)                              | -                                                                                                               | -                                                                                     | vf-schema: has extra fields `deletable` and `inScopeOf` are these for internal use?                                                                                                                                                                                                                                                                                                                                                                                                               |
| P, O  | [Process](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L196)                                                             | [observation](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/observation.gql#L155)               | [rea_process](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_process)                               | [observation](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/observation)     | vf-schema: Missing `plannedIn` What is `unplannedEvents`? For the inverse relationships, do we want to group all `Intent`s, `Commitment`s, and `EconomicEvent`s together in the `inputs` and `outputs`? How is `track` and `trace` being handled? dna: Has extra `before` and `after` fields. `planned_within` is present, despite no implementation (because it just points to an `entryHash`.) This is often placed in with Observation layer, or on the line between Observation and Planning. |
| P     | [Intent](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L139)                                                              | [planning](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/planning.gql#L94)                      | [rea_intent](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_intent)                                 | [planning](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/planning)           | vf-schema: Missing `provider`, `reciever`, `atLocation`. Has a `satisfiedBy` inverse map to `Satisfaction`'s `satisfies`.                                                                                                                                                                                                                                                                                                                                                                         |
| P     | [Proposed Intent](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L151)                                                     | [proposal](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/proposal.gql#L49)                      | [rea_proposed_intent](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_proposed_intent)               | [proposal](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/proposal)           |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| P     | [Proposal](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L145)                                                            | [proposal](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/proposal.gql#L16)                      | [rea_proposal](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_proposal)                             | [proposal](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/proposal)           | vf-schema: Missing `eligibleLocation`. Has a `publishes` inverse map to `ProposedIntent`'s `publishedIn`. zome: same.                                                                                                                                                                                                                                                                                                                                                                             |
| P     | [Proposed To](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L157)                                                         | [proposal.agent](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/bridging/proposal.agent.gql#L20) | [rea_proposed_to](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_proposed_to)                       | [proposal](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/proposal)           |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| P     | [Commitment](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L163)                                                          | [planning](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/planning.gql#L24)                      | [rea_commitment](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_commitment)                         | [planning](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/planning)           | vf-schema: Missing `atLocation` and `clauseOf`. Has `fullfilledBy` and `satisfies` inverse maps to `Fulfillment`'s`fulfill` and `Satisfation`'s `satisfiedBy`. zome: has `plan` instead of `planed_within`.                                                                                                                                                                                                                                                                                       |
| P     | [Satisfaction](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L169)                                                        | [planning](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/planning.gql#L188)                     | [rea_satisfaction](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_satisfaction)                     | [planning](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/planning)           | zome: allows `satisfied_by` to only be either one `EconomicEvent` or `Commitment`. Is this correct?                                                                                                                                                                                                                                                                                                                                                                                               |
| P     | [Agreement](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L175)                                                           | [agreement](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/agreement.gql#L19)                    | [rea_agreement](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_agreement)                           | [agreement](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/agreement)         |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| P     | [Claim](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L175)                                                               | [claim](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/claim.gql#L18)                            | -                                                                                                               | -                                                                                     |  Pospi has mentioned to me (Connor) that this has been de-prioritized  due to lack of pull for it from use cases ... is more speculative. Hence lack of implementation.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| O     | [Economic Resource](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L190)                                                   | [observation](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/observation.gql#L83)                | [rea_economic_resource](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_economic_resource)           | [observation](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/observation)     | vf-schema: Missing `currentLocation`. Has `contains`, `track`, `trace` maps as additions.                                                                                                                                                                                                                                                                                                                                                                                                         |
| O     | [dfc:ProductBatch](http://www.virtual-assembly.org/DataFoodConsortium/BusinessOntology)                                                                                   | [observation](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/observation.gql#L139)               | -                                                                                                               | -                                                                                     | vf-schema: Missing links to `identifies`, but that probably doesn't matter for our use case.                                                                                                                                                                                                                                                                                                                                                                                                      |
| O     | [Economic Event](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L202)                                                      | [observation](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/observation.gql#L19)                | [rea_economic_event](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_economic_event)                 | [observation](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/observation)     | vf-schema: Missing `realizationOf`, `image`, `provider`, `receiver`, `atLocation`, `toLocation`. Has `track` and `trace` going to `ProductionFlowItem`s. zome: Missing `to_location`.                                                                                                                                                                                                                                                                                                             |
| O     | [Appreciation](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L232)                                                        | [appreciation](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/appreciation.gql#L17)              | -                                                                                                               | -                                                                                     | Pospi has mentioned to me (Connor) that this has been de-prioritized  due to lack of pull for it from use cases ... is more speculative. Hence lack of implementation.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| P, O  | [Fulfillment](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L214)                                                         | [planning](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/planning.gql#L166)                     | [rea_fulfillment](https://github.com/holo-rea/holo-rea/tree/sprout/zomes/rea_fulfillment)                       | [observation](https://github.com/holo-rea/holo-rea/tree/sprout/dna_bundles/observation)     | !! Discrepancy between "layer" and "vf-schema" files. FIXME                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| O     | [Settlement](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L226)                                                          | [claim](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/claim.gql#L61)                            | -                                                                                                               | -                                                                                     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| O     | [Agent Relationship](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L208)                                                  | [agent](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/lib/schemas/agent.gql#L104)                           | -                                                                                                               | -                                                                                     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

There are internal system objects used to help specify the rules of logic around the actions:

* [Resource Effect](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L1278)
* [Contained Effect](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L68)
* [Location Effect](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L62)
* [Onhand Effect](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L56)
* [Input/Output](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L44)
* [Pairs With](https://lab.allmende.io/valueflows/valueflows/-/blob/master/release-doc-in-process/all_vf.TTL#L38)

In the gql version, these are just strings (need to learn more specifics).





## GraphQL Implementation

### Mutations

#### Implemented & Tested
- [x] createEconomicEvent
    - [x] fixed - `newInventoriedResource` `name` property is not persisted - [issue #202](https://github.com/holo-rea/holo-rea/issues/202)
- [x] createUnit
- [x] createProcess

#### Implemented & Not Yet Tested
- [x] createAgreement
- [x] updateAgreement
- [x] deleteAgreement
- [x] createCommitment
- [x] updateCommitment
- [x] deleteCommitment
- [x] updateEconomicEvent
- [x] deleteEconomicEvent
- [x] createFulfillment
- [x] updateFulfillment
- [x] deleteFulfillment
- [x] updateEconomicResource
- [x] createIntent
- [x] updateIntent
- [x] deleteIntent
- [x] updateProcess
- [x] deleteProcess
- [x] createProcessSpecification
- [x] updateProcessSpecification
- [x] deleteProcessSpecification
- [x] createProposal
- [x] updateProposal
- [x] deleteProposal
- [x] proposeIntent
- [x] deleteProposedIntent
- [x] proposeTo
- [x] deleteProposedTo
- [x] updateResourceSpecification
- [x] deleteResourceSpecification
- [x] createSatisfaction
- [x] updateSatisfaction
- [x] deleteSatisfaction
- [x] updateUnit
- [x] deleteUnit

#### Partially Implemented
- [x] createResourceSpecification
    - [ ] lacking `defaultUnitOfResource` - [issue #155](https://github.com/holo-rea/holo-rea/issues/155)

#### Has Minor Bug

#### Has Fatal Bug

#### Not Yet Implemented
- [ ] deleteEconomicResource - [issue #67](https://github.com/holo-rea/holo-rea/issues/67)
- [ ] createProductBatch - [issue #134](https://github.com/holo-rea/holo-rea/issues/134)
- [ ] updateProductBatch - [issue #134](https://github.com/holo-rea/holo-rea/issues/134)
- [ ] deleteProductBatch - [issue #134](https://github.com/holo-rea/holo-rea/issues/134)
- [ ] createPerson - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] updatePerson - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] deletePerson - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] createOrganization - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] updateOrganization - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] deleteOrganization - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] createAgentRelationship - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] updateAgentRelationship - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] deleteAgentRelationship - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] createAgentRelationshipRole - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] updateAgentRelationshipRole - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] deleteAgentRelationshipRole - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)



### Queries

#### Implemented & Tested
- [x] action
- [x] actions
- [x] unit
- [x] economicEvent

#### Implemented & Not Yet Tested
- [x] agreement
- [x] commitment
- [x] resourceSpecification
- [x] processSpecification
- [x] process
- [x] intent
- [x] fulfillment
- [x] satisfaction
- [x] proposal

__Has Partial Implementation__
- [x] myAgent
    - [ ] TODO: define what's lacking
- [x] agent
    - [ ] TODO: define what's lacking
- [x] economicResources
    - [ ] lacking pagination  - [issue #85](https://github.com/holo-rea/holo-rea/issues/85)
- [x] economicEvents
    - [ ] lacking pagination  - [issue #85](https://github.com/holo-rea/holo-rea/issues/85)
- [x] economicResource
    - [ ] `primaryAccountable` is not implemented - [issue #133](https://github.com/holo-rea/holo-rea/issues/133)

__Has Minor Bug__


__Has Fatal Bug__
- [ ] agents (response always gives empty array, wrongly - [issue #210](https://github.com/holo-rea/holo-rea/issues/210))

__Not Yet Implemented__
- [ ] proposals - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] satisfactions - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] fulfillments - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] intents - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] commitments - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] processes - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] productBatch - [issue #134](https://github.com/holo-rea/holo-rea/issues/134)
- [ ] productBatches - [issue #84](https://github.com/holo-rea/holo-rea/issues/84) and [issue #134](https://github.com/holo-rea/holo-rea/issues/134)
- [ ] units - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] processSpecifications - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] resourceSpecifications - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] agreements - [issue #84](https://github.com/holo-rea/holo-rea/issues/84)
- [ ] organization - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] organizations - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] person - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] people - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] agentRelationship - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] agentRelationships - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] agentRelationshipRole - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)
- [ ] agentRelationshipRoles - [issue #172](https://github.com/holo-rea/holo-rea/issues/172)

### Resolvers

(https://www.apollographql.com/docs/apollo-server/data/resolvers/)
Connor todo
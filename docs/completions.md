# holo-rea Graphql API Completions & Statuses

[![hackmd-github-sync-badge](https://hackmd.io/CWcN1gbER9ioLVy8xGUj1Q/badge)](https://hackmd.io/CWcN1gbER9ioLVy8xGUj1Q)

From the point-of-view of someone calling through Graphql, what is the overall status of each function that exists in the graphql schema. Many functions are not yet implemented so it is important to know that upfront, which are, and which aren't.

A filtered list of related github issues for tracking these work statuses, so that you can contribute, or report or discuss issues, can be found here: https://github.com/holo-rea/holo-rea/labels/graphql-api

## GraphQL Implementation

### Mutations

#### Implemented
- [x] createPerson
- [x] updatePerson
- [x] deletePerson
- [x] createOrganization
- [x] updateOrganization
- [x] deleteOrganization
- [x] createAgreement
- [x] updateAgreement
- [x] deleteAgreement
- [x] createCommitment
- [x] updateCommitment
- [x] deleteCommitment
- [x] createEconomicEvent
- [x] updateEconomicEvent
- [x] createFulfillment
- [x] updateFulfillment
- [x] deleteFulfillment
- [x] updateEconomicResource
- [x] createIntent
- [x] updateIntent
- [x] deleteIntent
- [x] createProcess
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
- [x] createResourceSpecification
    - [ ] lacking `defaultUnitOfResource` - [issue #155](https://github.com/h-rea/hrea/issues/155)
    - [ ] lacking `resourceClassifiedAs` - [issue #334](https://github.com/h-REA/hREA/issues/334)
- [x] updateResourceSpecification
- [x] deleteResourceSpecification
- [x] createSatisfaction
- [x] updateSatisfaction
- [x] deleteSatisfaction
- [x] createUnit
- [x] updateUnit
- [x] deleteUnit

#### Not Yet Implemented
- [ ] createAgentRelationship - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] updateAgentRelationship - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] deleteAgentRelationship - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] createAgentRelationshipRole - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] updateAgentRelationshipRole - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] deleteAgentRelationshipRole - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)

### Queries

#### Implemented
- [x] myAgent
- [x] agent
- [x] agents
- [x] organization
- [x] organizations
- [x] person
- [x] people
- [x] economicResource
- [x] economicResources
- [x] economicEvent
- [x] economicEvents
- [x] action
- [x] actions
- [x] unit
- [x] units
- [x] agreement
- [x] agreements
- [x] commitment
- [x] commitments
- [x] resourceSpecification
- [x] resourceSpecifications
- [x] processSpecification
- [x] processSpecifications
- [x] process
- [x] processes
- [x] intent
- [x] intents
- [x] fulfillment
- [x] fulfillments
- [x] satisfaction
- [x] satisfactions
- [x] proposal
- [x] proposals

__Not Yet Implemented__
- [ ] agentRelationship - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] agentRelationships - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] agentRelationshipRole - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] agentRelationshipRoles - [issue #321](https://github.com/holo-rea/holo-rea/issues/321)
- [ ] offers - [issue #322](https://github.com/h-REA/hREA/issues/322)
- [ ] requests - [issue #322](https://github.com/h-REA/hREA/issues/322)

### Resolvers

To see which relationships are NOT yet implemented, see here:
https://github.com/h-REA/hREA/issues/323#issue-1292415099

const associateMyAgentExtension = `
type Mutation  {
    "Associates the Agent identified by agentId with the currently authenticated user. Can only be used once."
    associateMyAgent(agentId: ID!): Boolean!
}
`;

const hasIds = `
type Measure {
    hasUnitId: ID!
}

type ResourceSpecification {
    defaultUnitOfResourceId: ID!
}

type Intent {
    providerId: ID!
    receiverId: ID!
}

type Commitment {
    providerId: ID!
    receiverId: ID!
    stageId: ID!
}

type EconomicEvent {
    providerId: ID!
    receiverId: ID!
    inputOfId: ID!
    outputOfId: ID!
}
`;

export { associateMyAgentExtension, hasIds };

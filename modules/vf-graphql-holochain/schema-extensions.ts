const associateMyAgentExtension = `
type Mutation  {
    "Associates the Agent identified by agentId with the currently authenticated user. Can only be used once."
    associateMyAgent(agentId: ID!): Boolean!
}
`;

export { associateMyAgentExtension };

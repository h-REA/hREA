import { EntryHash } from "@holochain/client"

export interface PagingParams {
    first?: number
    after?: string
    last?: number
    before?: string
}

interface SearchInput<QueryParamType> {
  params: QueryParamType,
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
export type RecipeProcessAddress = AddressableIdentifier
export type RecipeExchangeAddress = AddressableIdentifier
export type RecipeFlowAddress = AddressableIdentifier

export type CommitmentSearchInput = SearchInput<CommitmentQueryParam>
export type EconomicEventSearchInput = SearchInput<EconomicEventQueryParams>
export type FulfillmentSearchInput = SearchInput<FulfillmentQueryParams>
export type SatisfactionSearchInput = SearchInput<SatisfactionQueryParams>
export type ProcessSearchInput = SearchInput<ProcessQueryParams>
export type ResourceSpecificationSearchInput = SearchInput<ResourceSpecificationQueryParams>
export type EconomicResourceSearchInput = SearchInput<EconomicResourceQueryParams>
export type IntentSearchInput = SearchInput<IntentQueryParams>
export type PlanSearchInput = SearchInput<PlanQueryParams>
export type ProposalSearchInput = SearchInput<ProposalQueryParams>
export type AgentSearchInput = SearchInput<AgentQueryParams>
export type ProposedIntentSearchInput = SearchInput<ProposedIntentQueryParams>
export type RecipeFlowSearchInput = SearchInput<RecipeFlowQueryParams>

interface CommitmentQueryParam {
    inputOf?: ProcessAddress,
    outputOf?: ProcessAddress,
    fulfilledBy?: FulfillmentAddress,
    satisfies?: SatisfactionAddress,
    clauseOf?: AgreementAddress,
    independentDemandOf?: PlanAddress,
    plannedWithin?: PlanAddress,
    inScopeOf?: AgentAddress,
    provider?: AgentAddress,
    receiver?: AgentAddress,
}

interface EconomicEventQueryParams {
    inputOf?: ProcessAddress,
    outputOf?: ProcessAddress,
    satisfies?: IntentAddress,
    fulfills?: CommitmentAddress,
    realizationOf?: AgreementAddress,
    affects?: EconomicResourceAddress,
    inScopeOf?: AgentAddress,
    provider?: AgentAddress,
    receiver?: AgentAddress,
}

interface FulfillmentQueryParams {
    fulfills?: CommitmentAddress,
    fulfilledBy?: EconomicEventAddress,
}
interface SatisfactionQueryParams {
    satisfies?: IntentAddress,
    satisfiedBy?: CommitmentAddress,
}

interface ProcessQueryParams {
    observedInputs?: EconomicEventAddress,
    observedOutputs?: EconomicEventAddress,
    unplannedEconomicEvents?: EconomicEventAddress,
    committedInputs?: CommitmentAddress,
    committedOutputs?: CommitmentAddress,
    intendedInputs?: IntentAddress,
    intendedOutputs?: IntentAddress,
    workingAgents?: AgentAddress,
    plannedWithin?: PlanAddress,
    inScopeOf?: AgentAddress,
}

interface ResourceSpecificationQueryParams {
    conformingResources?: EconomicResourceAddress,
}
interface EconomicResourceQueryParams {
    contains?: EconomicResourceAddress,
    containedIn?: EconomicResourceAddress,
    conformsTo?: ResourceSpecificationAddress,
    affectedBy?: EconomicEventAddress,
    primaryAccountable?: AgentAddress,
}
interface IntentQueryParams {
    inputOf?: ProcessAddress,
    outputOf?: ProcessAddress,
    satisfiedBy?: SatisfactionAddress,
    proposedIn?: ProposedIntentAddress,
    inScopeOf?: AgentAddress,
    provider?: AgentAddress,
    receiver?: AgentAddress,
    publishedIn?: ProposedIntentAddress,
}

interface ProposedIntentQueryParams {
    publishedIn?: ProposalAddress,
}

interface PlanQueryParams {
    inScopeOf?: AgentAddress,
}

interface ProposalQueryParams {
    inScopeOf?: AgentAddress,
}

interface AgentQueryParams {
    agentType?: string,
}

interface RecipeFlowQueryParams {
    recipeInputOf?: RecipeProcessAddress,
    recipeOutputOf?: RecipeProcessAddress,
    recipeClauseOf?: RecipeExchangeAddress,
    recipeReciprocalClauseOf?: RecipeExchangeAddress,
}
import { EntryHash } from "@sprillow-connor/holochain-client"
import { AgentAddress, AgreementAddress, CommitmentAddress, EconomicEventAddress, EconomicResourceAddress, FulfillmentAddress, IntentAddress, PlanAddress, ProcessAddress, ProposedIntentAddress, ResourceSpecificationAddress, SatisfactionAddress } from "../types";

// this type name
// matches the Rust side type name
export interface PagingParams {
    // TODO: forwards pagination
    // first: number
    // after: string
    last?: number,
    before?: EntryHash
}

interface SearchInput<QueryParamType> {
  params: QueryParamType,
}

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

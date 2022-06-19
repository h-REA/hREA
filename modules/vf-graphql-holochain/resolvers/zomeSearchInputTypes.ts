import { AgentAddress, AgreementAddress, CommitmentAddress, EconomicEventAddress, EconomicResourceAddress, FulfillmentAddress, IntentAddress, PlanAddress, ProcessAddress, ProposedIntentAddress, ResourceSpecificationAddress, SatisfactionAddress } from "../types";

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

interface CommitmentQueryParam {
    inputOf?: ProcessAddress,
    outputOf?: ProcessAddress,
    fulfilledBy?: FulfillmentAddress,
    satisfies?: SatisfactionAddress,
    clauseOf?: AgreementAddress,
    independentDemandOf?: PlanAddress,
    plannedWithin?: PlanAddress,
    inScopeOf?: AgentAddress,
}

interface EconomicEventQueryParams {
    inputOf?: ProcessAddress,
    outputOf?: ProcessAddress,
    satisfies?: IntentAddress,
    fulfills?: CommitmentAddress,
    realizationOf?: AgreementAddress,
    affects?: EconomicResourceAddress,
    inScopeOf?: AgentAddress,
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
    inputs?: EconomicEventAddress,
    outputs?: EconomicEventAddress,
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
}

interface PlanQueryParams {
    inScopeOf?: AgentAddress,
}

interface ProposalQueryParams {
    inScopeOf?: AgentAddress,
}

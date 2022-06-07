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

interface CommitmentQueryParam {
    inputOf?: ProcessAddress,
    outputOf?: ProcessAddress,
    fulfilledBy?: FulfillmentAddress,
    satisfies?: SatisfactionAddress,
    clauseOf?: AgreementAddress,
    independentDemandOf?: PlanAddress,
    plannedWithin?: PlanAddress,
}

interface EconomicEventQueryParams {
    inputOf?: ProcessAddress,
    outputOf?: ProcessAddress,
    satisfies?: IntentAddress,
    fulfills?: CommitmentAddress,
    realizationOf?: AgreementAddress,
    affects?: EconomicResourceAddress,
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
}

interface ResourceSpecificationQueryParams {
    conformingResources?: EconomicResourceAddress,
}
interface EconomicResourceQueryParams {
    contains?: EconomicResourceAddress,
    containedIn?: EconomicResourceAddress,
    conformsTo?: ResourceSpecificationAddress,
    affectedBy?: EconomicEventAddress,
}
interface IntentQueryParams {
    inputOf?: ProcessAddress,
    outputOf?: ProcessAddress,
    satisfiedBy?: SatisfactionAddress,
    proposedIn?: ProposedIntentAddress,
}
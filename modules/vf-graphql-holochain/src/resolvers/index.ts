import Query from "../queries/index.js"
import Mutation from "../mutations/index.js"
import Agent from "./agent.js"
import { getOne } from "../queries/helpers.js"
import { getCollection, getCollectionLinks, getAction } from "../util.js"
import { encodeHashToBase64 } from "@holochain/client"
import { GraphQLResolveInfo } from "graphql";
import { getLastUpdateTime } from "../store.js"

export const generateResolvers = (cell: any) => {
    const get = async (entryType, id, info: GraphQLResolveInfo) => {
        if (!id) return null;
        const isOnlyIdRequested = info.fieldNodes[0].selectionSet?.selections.every(
            selection => selection.kind === "Field" && selection.name.value === "id"
        );
        if (isOnlyIdRequested) return { id: encodeHashToBase64(id) };
        const output = await getOne(cell, entryType, { id: encodeHashToBase64(id) });
        return output;
    }
    const getMany = async (func, fromId, info: GraphQLResolveInfo) => {
        if (!fromId) return null;
        const isOnlyIdOrRevisionIdRequested = info.fieldNodes[0].selectionSet?.selections.every(
            selection => selection.kind === "Field" && (selection.name.value === "id" || selection.name.value === "revisionId")
        );
        if (isOnlyIdOrRevisionIdRequested) {
            const links = await getCollectionLinks(cell, func, fromId);
            return links.map((link: any) => {
                return {
                    id: encodeHashToBase64(link.tag),
                    revisionId: encodeHashToBase64(link.target)
                }
            })
        }
        return await getCollection(cell, func, fromId);
    }
    const getList = async (entryType, list, info: GraphQLResolveInfo) => {
        return (await Promise.all((list || [])
                    .map((address)=>get(entryType, address, info))))
                    .filter((item) => item !== null)
    }

    const getMeta = async (record) => {
        return {
            retrievedRevision: {
                id: record.id,
                time: getLastUpdateTime(record.id),
            }
        }
    }

    const res = Object.assign({
        Query: Query(cell),
        Mutation: Mutation(cell),
        Agent: Agent(cell),
        Agreement: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            commitments: async function (record, args, context, info) {
                return await getMany('get_rea_commitments_for_rea_agreement', record.id, info)
            },
            economicEvents: async function (record, args, context, info) {
                return await getMany('get_rea_economic_events_for_rea_agreement', record.id, info)
            },
        },
        Commitment: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            outputOf: async function (record, args, context, info) { return get('process', record.outputOf, info) },
            inputOf: async function (record, args, context, info) { return get('process', record.inputOf, info) },
            receiver: async function (record, args, context, info) { return get('agent', record.receiver, info) },
            provider: async function (record, args, context, info) { return get('agent', record.provider, info) },
            clauseOf: async function (record, args, context, info) { return get('agreement', record.clauseOf, info) },
            plannedWithin: async function (record, args, context, info) { return get('plan', record.plannedWithin, info) },
            independentDemandOf: async function (record, args, context, info) { return get('plan', record.independentDemandOf, info) },
            fulfilledBy: async function (record, args, context, info) { return getMany('get_fulfilling_economic_events_for_commitment', record.id, info) },
            satisfies: async function (record, args, context, info) { return get('intent', record.satisfies, info) },
            action: function (record, args, context, info) { return getAction(record.action) },
        },
        EconomicEvent: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            resourceInventoriedAs: async function (record, args, context, info) { return get('economic_resource', record.resourceInventoriedAs, info) },
            toResourceInventoriedAs: async function (record, args, context, info) { return get('economic_resource', record.toResourceInventoriedAs, info) },
            inputOf: async function (record, args, context, info) { return get('economic_event', record.inputOf, info) },
            outputOf: async function (record, args, context, info) { return get('economic_event', record.outputOf, info) },
            provider: async function (record, args, context, info) { return get('agent', record.provider, info) },
            receiver: async function (record, args, context, info) { return get('agent', record.receiver, info) },
            fulfills: async function (record, args, context, info) { return getList('commitment', record.fulfills, info) },
            satisfies: async function (record, args, context, info) { return getList('intent', record.satisfies, info) },
            resourceConformsTo: async function (record, args, context, info) { return get('resource', record.resourceConformsTo, info) },
            action: function (record, args, context, info) { return getAction(record.action) },
            realizationOf: async function (record, args, context, info) { return get('agreement', record.realizationOf, info) },
        },
        EconomicResource: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            containedIn: async function (record, args, context, info) {
                return getMany('get_rea_economic_resources_for_rea_economic_resource', record.contained_in, info)
            },
            contains: async function (record, args, context, info) {
                return getMany('get_rea_economic_resources_for_rea_economic_resource', record.id, info)
            },
            conformsTo: async function (record, args, context, info) { return get('resource_specification', record.conformsTo, info) },
            stage: async function (record, args, context, info) { return get('process_specification', record.stage, info) },
            state: function (record, args, context, info) { return getAction(record.state) },
            unitOfEffort: async function (record, args, context, info) { return get('unit', record.unitOfEffort, info) },
            primaryAccountable: async function (record, args, context, info) { return get('agent', record.primaryAccountable, info) },
        },
        Intent: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            satisfiedBy: async function (record, args, context, info) {
                return getMany('get_satisfying_comitments_for_rea_intent', record.id, info)
            },
            observedBy: async function (record, args, context, info) {
                return getMany('get_satisfying_economic_events_for_rea_intent', record.id, info)
            },
            provider: async function (record, args, context, info) { return get('agent', record.provider, info) },
            receiver: async function (record, args, context, info) { return get('agent', record.receiver, info) },
            inputOf: async function (record, args, context, info) { return get('process', record.inputOf, info) },
            outputOf: async function (record, args, context, info) { return get('process', record.outputOf, info) },
            // publishedIn: async function (record, args, context, info) { return get('proposed_intent', record.publishedIn, info) },
            resourceConformsTo: async function (record, args, context, info) { return get('resource_specification', record.resourceConformsTo, info) },
            action: function (record, args, context, info) { return getAction(record.action) },
        },
        Measure: {
            hasUnit: async function (record, args, context, info) { 
                const unit = await get('unit', record.hasUnit, info)
                return unit ? unit : null;
            },
        },
        Plan: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            processes: async function (record, args, context, info) { 
                return getMany('get_rea_processes_for_rea_plan', record.id, info) 
            },
            independentDemands: async function (record, args, context, info) {
                return getMany('get_independent_demands_for_rea_plan', record.id, info) 
            },
            nonProcessCommitments: async function (record, args, context, info) {
                return getMany('get_rea_commitments_for_rea_plan', record.id, info) 
            },
            inScopeOf: async function (record, args, context, info) { return getList('agent', record.inScopeOf, info) },
        },
        Process: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            observedInputs: async function (record, args, context, info) {
                return getMany('get_rea_economic_event_inputs_for_rea_process', record.id, info)
            },
            observedOutputs: async function (record, args, context, info) {
                return getMany('get_rea_economic_event_outputs_for_rea_process', record.id, info)
            },
            committedInputs: async function (record, args, context, info) {
                return getMany('get_inputs_for_rea_process', record.id, info)
            },
            committedOutputs: async function (record, args, context, info) {
                return getMany('get_outputs_for_rea_process', record.id, info) 
            },
            intendedInputs: async function (record, args, context, info) {
                return getMany('get_rea_intents_for_rea_process_inputs', record.id, info) 
            },
            intendedOutputs: async function (record, args, context, info) {
                return getMany('get_rea_intents_for_rea_process_outputs', record.id, info) 
            },
            basedOn: async function (record, args, context, info) { return getOne(cell, 'process_specification', { id: record.basedOn }) },
            plannedWithin: async function (record, args, context, info) { return get('plan', record.plannedWithin, info) },
            inScopeOf: async function (record, args, context, info) { return getList('agent', record.inScopeOf, info) }
        },
        Proposal: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            publishes: async function (record, args, context, info) { return getList('intent', record.publishes, info) },
            reciprocal: async function (record, args, context, info) { return getList('intent', record.publishes, info) },
            proposedTo: async function (record, args, context, info) { return getList('agent', record.proposedTo, info) },
            inScopeOf: async function (record, args, context, info) { return getList('agent', record.inScopeOf, info) },
        },
        RecipeExchange: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            recipeClauses: async function (record, args, context, info) {
                return getMany('get_rea_recipe_clauses_for_rea_recipe_exchange', record.id, info)
            },
            recipeReciprocalClauses: async function (record, args, context, info) {
                return getMany('get_rea_recipe_reciprocal_clauses_for_rea_recipe_exchange', record.id, info)
            },
        },
        RecipeFlow: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            recipeInputOf: async function (record, args, context, info) { return get('recipe_process', record.recipeInputOf, info) },
            recipeOutputOf: async function (record, args, context, info) { return get('recipe_process', record.recipeOutputOf, info) },
            recipeClauseOf: async function (record, args, context, info) { return get('recipe_exchange', record.recipeClauseOf, info) },
            recipeReciprocalClauseOf: async function (record, args, context, info) { return get('recipe_exchange', record.recipeReciprocalClauseOf, info) },
            resourceConformsTo: async function (record, args, context, info) { return get('resource_specification', record.resourceConformsTo, info) },
            stage: async function (record, args, context, info) { return get('process_specification', record.stage, info) },
            action: function (record, args, context, info) { return getAction(record.action) },
        },
        RecipeProcess: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            recipeInputs: async function (record, args, context, info) {
                return getMany('get_rea_recipe_flow_inputs_for_rea_recipe_process', record.id, info)
            },
            recipeOutputs: async function (record, args, context, info) {
                return getMany('get_rea_recipe_flow_outputs_for_rea_recipe_process', record.id, info)
            },
            processConformsTo: async function (record, args, context, info) { return get('process_specification', record.processConformsTo, info) },
        },
        ResourceSpecification: {
            meta: async function (record, args, context, info) {return getMeta(record)},
            // conformingResources: async function (record, args, context, info) {
            //     return getMany('economic_resource', '', record.id, info)
            // },
            defaultUnitOfResource: async function (record, args, context, info) { return get('unit', record.defaultUnitOfResource, info) },
            defaultUnitOfEffort: async function (record, args, context, info) { return get('unit', record.defaultUnitOfEffort, info) },
        },
    })
    return res
}
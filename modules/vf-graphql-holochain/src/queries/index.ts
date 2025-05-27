import Action from "./action.js"
import Agent from "./agent.js"
import { getAll, getOne } from "./helpers.js"

export default (cell: any) => {
    return Object.assign({
        ...Agent(cell),
        ...Action(cell),
        agreements: async (root, args) => { 
            return await getAll(cell, "agreement", args) },
        agreement: async (root, args) => { 
            return await getOne(cell, "agreement", args) },
        commitments: async (root, args) => { return await getAll(cell, "commitment", args) },
        commitment: async (root, args) => { return await getOne(cell, "commitment", args) },
        economicEvents: async (root, args) => { return await getAll(cell, "economic_event", args) },
        economicEvent: async (root, args) => { return await getOne(cell, "economic_event", args) },
        economicResources: async (root, args) => { return await getAll(cell, "economic_resource", args) },
        economicResource: async (root, args) => { return await getOne(cell, "economic_resource", args) },
        intents: async (root, args) => { return await getAll(cell, "intent", args) },
        intent: async (root, args) => { return await getOne(cell, "intent", args) },
        plans: async (root, args) => { return await getAll(cell, "plan", args) },
        plan: async (root, args) => { return await getOne(cell, "plan", args) },
        processes: async (root, args) => { return await getAll(cell, "process", args) },
        process: async (root, args) => { return await getOne(cell, "process", args) },
        processSpecifications: async (root, args) => { return await getAll(cell, "process_specification", args) },
        processSpecification: async (root, args) => { return await getOne(cell, "process_specification", args) },
        proposals: async (root, args) => { return await getAll(cell, "proposal", args) },
        proposal: async (root, args) => { return await getOne(cell, "proposal", args) },
        recipeExchanges: async (root, args) => { return await getAll(cell, "recipe_exchange", args) },
        recipeExchange: async (root, args) => { return await getOne(cell, "recipe_exchange", args) },
        recipeFlows: async (root, args) => { return await getAll(cell, "recipe_flow", args) },
        recipeFlow: async (root, args) => { return await getOne(cell, "recipe_flow", args) },
        recipeProcesses: async (root, args) => { return await getAll(cell, "recipe_process", args) },
        recipeProcess: async (root, args) => { return await getOne(cell, "recipe_process", args) },
        resourceSpecifications: async (root, args) => { return await getAll(cell, "resource_specification", args) },
        resourceSpecification: async (root, args) => { return await getOne(cell, "resource_specification", args) },
        units: async (root, args) => { return await getAll(cell, "unit", args) },
        unit: async (root, args) => { return await getOne(cell, "unit", args) },
    })
}
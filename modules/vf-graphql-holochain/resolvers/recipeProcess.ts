/**
 * Recipe Process record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ById, ByRevision, ProcessSpecificationAddress, AddressableIdentifier, RecipeFlowAddress } from '../types.js'
import { extractEdges, mapZomeFn } from '../connection.js'

import {
  RecipeProcess,
  RecipeProcessResponse,
  Action,
  ProcessSpecification,
  ProcessSpecificationResponse,
  RecipeFlow,
  RecipeFlowResponse,
  RecipeFlowConnection,
} from '@leosprograms/vf-graphql'

import { RecipeFlowSearchInput } from './zomeSearchInputTypes.js'

// const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  // const hasAction = -1 !== enabledVFModules.indexOf(VfModule.Action)
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasProcessSpecification = -1 !== enabledVFModules.indexOf(VfModule.ProcessSpecification)
  
  const readRevision = mapZomeFn<ByRevision, RecipeProcessResponse>(dnaConfig, conductorUri, 'combined', 'recipe_process', 'get_revision')
  // const readAction = mapZomeFn<ById, Action>(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readProcessSpecification = mapZomeFn<ReadParams, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'combined', 'process_specification', 'get_process_specification')
  // const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const readRecipeFlows = mapZomeFn<RecipeFlowSearchInput, RecipeFlowConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'query_recipe_flows')

  return Object.assign(
    {
      recipeInputs: async (record: RecipeProcess): Promise<RecipeFlow[]> => {
        const recipeFlows = await readRecipeFlows({ params: { recipeInputOf: record.id } })
        return extractEdges(recipeFlows)
      },
      recipeOutputs: async (record: RecipeProcess): Promise<RecipeFlow[]> => {
        const recipeFlows = await readRecipeFlows({ params: { recipeOutputOf: record.id } })
        return extractEdges(recipeFlows)
      }
    },    
    (hasHistory ? {
      revision: async (record: RecipeProcess, args: { revisionId: AddressableIdentifier }): Promise<RecipeProcess> => {
        return (await readRevision(args)).recipeProcess
      },
    } : {}),
    // (hasAction ? {
    //   action: async (record: { action: AddressableIdentifier }): Promise<Action> => {
    //     return (await readAction({ id: record.action }))
    //   },
    // } : {}),
    (hasProcessSpecification ? {
      processConformsToId: async (record: { processConformsTo: ProcessSpecificationAddress }): Promise<ProcessSpecificationAddress> => {
        return record.processConformsTo
      },
      processConformsTo: async (record: { processConformsTo: ProcessSpecificationAddress }): Promise<ProcessSpecification | undefined> => {
        try {
          return (await readProcessSpecification({ address: record.processConformsTo })).processSpecification
        } catch (e) {
          return undefined
        }
      },
    } : {}),
    
    // (hasProcess ? {
    //   inputOf: async (record: RecipeProcess): Promise<Process> => {
    //     const results = await readProcesses({ params: { intendedInputs: record.id } })
    //     return results.edges.pop()!['node']
    //   },

    //   outputOf: async (record: RecipeProcess): Promise<Process> => {
    //     const results = await readProcesses({ params: { intendedOutputs: record.id } })
    //     return results.edges.pop()!['node']
    //   },
    // } : {}),
  )
}

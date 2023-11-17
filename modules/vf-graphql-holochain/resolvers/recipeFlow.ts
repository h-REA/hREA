/**
 * Recipe Flow record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ById, ByRevision, ProposedIntentAddress, ResourceSpecificationAddress, ProcessSpecificationAddress, RecipeProcessAddress, AddressableIdentifier, AgentAddress } from '../types.js'
import { extractEdges, mapZomeFn } from '../connection.js'

import {
  Maybe,
  Agent,
  RecipeFlow,
  RecipeFlowResponse,
  Satisfaction,
  Process,
  ResourceSpecification,
  ProcessSpecification,
  ProposedIntent,
  Action,
  SatisfactionConnection,
  ProcessConnection,
  ProposedIntentResponse,
  ResourceSpecificationResponse,
  ProcessSpecificationResponse,
  AccountingScope,
  RecipeProcess,
  RecipeProcessResponse,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent.js'
import { ProcessSearchInput, SatisfactionSearchInput } from './zomeSearchInputTypes.js'

const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasResourceSpecification = -1 !== enabledVFModules.indexOf(VfModule.ResourceSpecification)
  const hasProcessSpecification = -1 !== enabledVFModules.indexOf(VfModule.ProcessSpecification)
  const hasAction = -1 !== enabledVFModules.indexOf(VfModule.Action)
  const hasRecipeInput = -1 !== enabledVFModules.indexOf(VfModule.RecipeProcess)
  const hasRecipeOutput = -1 !== enabledVFModules.indexOf(VfModule.RecipeProcess)  
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)

  const readResourceSpecification = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readProcessSpecification = mapZomeFn<ReadParams, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_process_specification')
  const readAction = mapZomeFn<ById, Action>(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readRecipeProcess = mapZomeFn<ReadParams, RecipeProcessResponse>(dnaConfig, conductorUri, 'specification', 'planning', 'get_recipe_process')
  const readRevision = mapZomeFn<ByRevision, RecipeFlowResponse>(dnaConfig, conductorUri, 'planning', 'recipe_flow', 'get_revision')

  return Object.assign(
    (hasResourceSpecification ? {
      resourceConformsTo: async (record: { resourceConformsTo: ResourceSpecificationAddress }): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
      },
    } : {}),
    (hasProcessSpecification ? {
      processConformsTo: async (record: { stage: ProcessSpecificationAddress }): Promise<ProcessSpecification> => {
        return (await readProcessSpecification({ address: record.stage })).processSpecification
      },
    } : {}),
    (hasAction ? {
      action: async (record: { action: AddressableIdentifier }): Promise<Action> => {
        return (await readAction({ id: record.action }))
      },
    } : {}),
    (hasRecipeInput ? {
      recipeInputOf: async (record: { recipeInputOf: RecipeProcessAddress }): Promise<RecipeProcess> => {
        return (await readRecipeProcess({ address: record.recipeInputOf })).recipeProcess
      },
    } : {}),
    (hasRecipeOutput ? {
      recipeOutputOf: async (record: { recipeOutputOf: RecipeProcessAddress }): Promise<RecipeProcess> => {
        return (await readRecipeProcess({ address: record.recipeOutputOf })).recipeProcess
      },
    } : {}),
    (hasHistory ? {
      revision: async (record: RecipeFlow, args: { revisionId: AddressableIdentifier }): Promise<RecipeFlow> => {
        return (await readRevision(args)).recipeFlow
      },
    } : {}),
  )
}

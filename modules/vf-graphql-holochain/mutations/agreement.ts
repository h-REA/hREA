/**
 * Agreement CRUD operations
 *
 * @package: Holo-REA
 * @since:   2020-06-19
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  AgreementCreateParams,
  AgreementUpdateParams,
  AgreementResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  agreement: AgreementCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<AgreementResponse>

export interface UpdateArgs {
  agreement: AgreementUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<AgreementResponse>

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'agreement', 'agreement', 'create_agreement')
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'agreement', 'agreement', 'update_agreement')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'agreement', 'agreement', 'delete_agreement')

  const createAgreement: createHandler = async (root, args) => {
    // :SHONK: Inject current time as `created` if not present.
    //         Not to spec, but needed for entropy to avoid hash collisions.
    if (!args.agreement.created) {
      args.agreement.created = new Date()
    }
    return runCreate(args)
  }

  const updateAgreement: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteAgreement: deleteHandler = async (root, args) => {
    return runDelete({ address: args.id })
  }

  return {
    createAgreement,
    updateAgreement,
    deleteAgreement,
  }
}

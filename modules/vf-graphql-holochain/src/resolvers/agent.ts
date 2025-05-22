import type { Agent, PlanConnection } from '@valueflows/vf-graphql'
import { paginateCollection } from '../util'

export default (cell: any) => {
    return Object.assign({
        __resolveType(obj: any) {
            return obj.agentType || "Organization"
        }
    })
}
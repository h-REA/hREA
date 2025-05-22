import { Action } from '@valueflows/vf-graphql'

export default (cell: any) => {
    return {
        actions: async (root, args): Promise<Array<Action>> => {
            const res = await cell.callZome({
                zome_name: "hrea",
                fn_name: "get_all_actions",
                payload: null,
            })
            return res
        },
        action: async (root, args): Promise<Action> => {
            const res = await cell.callZome({
                zome_name: "hrea",
                fn_name: "get_action",
                payload: args.id,
            })
            return res
        }
    }
}
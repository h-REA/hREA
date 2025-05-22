import { createHolochainSchema } from '@valueflows/vf-graphql-holochain';
import { ApolloServer } from 'apollo-server';
import getPort from 'get-port';

export async function setupTest(scenario) {
    const testAppPath = process.cwd() + "/../workdir/hrea.happ";
    const appSource = { appBundleSource: { path: testAppPath } };
    const [alice] = await scenario.addPlayersWithApps([appSource]);
    // @ts-ignore
    const schema = createHolochainSchema({cell: alice.cells[0]});
    const server = new ApolloServer({ schema });
    const port = await getPort();
    server.listen(port).then(({ url }) => console.log(`Server ready at ${url}`));
    return server;
}

export async function graphQL(server, gql: string, variables: any = {}) {
    const result = await server.executeOperation({
        query: gql,
        variables,
    });
    return result;
}
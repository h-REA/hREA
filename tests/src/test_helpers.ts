import { createHolochainSchema } from '@valueflows/vf-graphql-holochain';
import { ApolloServer } from 'apollo-server';
import getPort from 'get-port';

export async function setupTest(scenario) {
    console.log("Setting up test environment");
    const testAppPath = process.cwd() + "/../workdir/hrea.happ";
    console.log("Test app path:", testAppPath);
    const appSource = { appBundleSource: {  type: "path" as const, value: testAppPath } };
    console.log("App source:", appSource);
    const [alice] = await scenario.addPlayersWithApps([appSource]);
    console.log("Alice's cell:", alice.cells[0]);
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
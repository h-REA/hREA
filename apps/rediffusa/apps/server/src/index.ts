import { ApolloServer, BaseContext, ContextFunction } from "@apollo/server";
import {
  StandaloneServerContextFunctionArgument,
  startStandaloneServer,
} from "@apollo/server/standalone";
import { loadFiles } from "@graphql-tools/load-files";
import { Neo4jGraphQL } from "@neo4j/graphql";
import neo4j from "neo4j-driver";
import * as dotenv from "dotenv";

dotenv.config();

const driver = neo4j.driver(
  process.env.DB_URI || "",
  neo4j.auth.basic(process.env.DB_USER || "", process.env.DB_PASSWORD || "")
);

const context: ContextFunction<
  [StandaloneServerContextFunctionArgument],
  BaseContext
> = async ({ req }) => ({ req, executionContext: driver });

const typeDefs = await loadFiles("src/graphql/**/*.graphql");

const neoSchema = new Neo4jGraphQL({
  typeDefs,
  driver,
});

const start = async () => {
  const schema = await neoSchema.getSchema();

  const server = new ApolloServer<BaseContext>({
    schema,
  });

  const { url } = await startStandaloneServer(server, {
    listen: { port: 4000 },
  });

  console.log("listening at: ", url);
};

start();

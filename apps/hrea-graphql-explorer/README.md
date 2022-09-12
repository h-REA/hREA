# hREA GraphQL explorer UI

Built using [Create React App](https://facebook.github.io/create-react-app/) code generator & [GraphiQL](https://github.com/graphql/graphiql).

## Important files:

*(in `src/` directory)*

- `App.tsx` contains a useful example of instantiating an [Apollo link](https://www.npmjs.com/package/apollo-link) for a schema. It is also where to define the default query that displays when opening the UI.
- `CustomArgs.ts` is for the [OneGraph GraphQL explorer](https://www.onegraph.com/blog/2019/01/24/How_OneGraph_onboards_users_new_to_GraphQL.html) UI, and determines the defaults used when de/selecting relationship fields in the explorer interface. This provides a simplified and approachable way for newcomers to interact with the GraphQL query interface & explore HoloREA.

# CHANGELOG

## hApp `0.1.0-beta.1`, NPM modules `0.0.1-alpha.14`

- Indexing fixes:
	- Storage efficiency fixes for semantic indexing logic
	- Fixed bugs in arbitrary time ordering logic for indexes from locally-adjacent zomes in the same DNA
	- Genericised integrity zomes for indexing such that multiple controller zomes for different record types can all reference the same integrity zome. This reduces boilerplate significantly.
	- Changed internal architecture of time indexing system to fix duplicate results and errors being returned due to looping references occurring on overlapping start periods (previously, "2023" and "Jan 2023" were the same node).
- EconomicResource:
	- Fixed ordering of contained/contains EconomicResources. The last *indexed* resource will now be the first retrieved.
- Client modules:
	- Connection introspection logic for the GraphQL adapter updated for Holochain Beta RC (`@holochain/client` v`0.11.9`). **An admin websocket connection is now required in order to initialise the connection parameters.** We are hopeful this will be a temporary situation- see https://github.com/holochain/holochain/issues/1746
- Updates for compatibility with deprecated methods from Rust `chrono` crate
- Updated Nix environment configuration, which now requires Nix v2.4
- Various other maintenance / infrastructural updates for Holochain Beta RC

While the external API remains compatible, these changes are backwards-incompatible with the previous version's data structures and a reset & fresh installation of any DHTs is necessary to upgrade.

## happ-0.1.0-beta

First stable API release, considered the starting point for integrations.

Compatible with the NPM modules [`@vf-ui/graphql-client-holochain`](https://www.npmjs.com/package/@vf-ui/graphql-client-holochain) and [`@valueflows/vf-graphql-holochain`](https://www.npmjs.com/package/@valueflows/vf-graphql-holochain) at version `0.0.1-alpha.13`. Prior versions of these modules were published during the (unstable) alpha testing phase down to `0.0.1-alpha.1` and should be considered 'unstable'.

Previous unstable alpha versions of the hREA Holochain components also exist from `0.0.1-alpha.1` through to `0.0.1-alpha.7`.

# CHANGELOG

## hApp 0.1.4-beta (unreleased)

- Fixed bug in core library for handling of unique manually-assigned ("anchored") identifiers. `Unit` records are no longer returned multiple times in `read_all_units` if the same `Unit` is continually recreated.
- Updated development environment to execute against Holochain 0.1.5-beta, but no changes to linked HDK/HDI module versions.

## NPM modules 0.0.1-alpha.21

- Republishing of 0.0.1-alpha.17-20 to fix additional misconfiguration of `@valueflows/vf-graphql-holochain` as an older ES6 module.

## NPM modules 0.0.1-alpha.18..20 **(broken)** (`@vf-ui/graphql-client-holochain` only)

- Republishing of 0.0.1-alpha.17 to fix missing `"type":"module"` specifier in GraphQL Client module manifest, causing linking issues in modern bundlers.
- 0.0.1-alpha.20 fixes additional nonstandard ESModule import in GraphQL Adapter module code.

## hApp 0.1.3-beta

- Updated all Holochain dependencies for latest HDK & HDI libraries. Tested compatible with Holochain Beta 0.1.3, considered backwards-compatible to Holochain 0.1.0.

## hApp 0.1.2-beta, NPM modules 0.0.1-alpha.17

- **Breaking:** updated all Holochain dependencies to 0.1.0, the first beta release.
	- Since this involves changes to the internal zome storage data format, these changes are backwards-incompatible with the previous version's data structures and a reset & fresh installation of any DHTs is necessary to upgrade a running system.
- Updated `@holochain/client` version used in GraphQL resolvers to 0.12.0
- Refix for `@vf-ui/graphql-client-holochain` to avoid importing `react` in Apollo dependencies.

## NPM modules 0.0.1-alpha.16

- Updated `@valueflows/vf-graphql` to 0.9.0-alpha.9. [changelog](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql/-/blob/sprout/CHANGELOG.md#090-alpha9)
	- Implemented a new `Decimal` GraphQL type using `big.js` for parsing numerical values.
- Updated `@graphql-tools/schema` and `@graphql-tools/merge` to latest versions.
- (failed) patch for `@vf-ui/graphql-client-holochain` to avoid importing `react` in Apollo dependencies.

## NPM modules 0.0.1-alpha.15

- Updated `@holochain/client` to 0.11.14.

## hApp 0.1.1-beta, NPM modules 0.0.1-alpha.14

- Indexing fixes:
	- Storage efficiency fixes for semantic indexing logic
	- Fixed bugs in arbitrary time ordering logic for indexes from locally-adjacent zomes in the same DNA
	- Genericised integrity zomes for indexing such that multiple controller zomes for different record types can all reference the same integrity zome. This reduces boilerplate significantly.
	- Changed internal architecture of time indexing system to fix duplicate results and errors being returned due to looping references occurring on overlapping start periods (previously, "2023" and "Jan 2023" were the same node).
- EconomicResource:
	- Fixed ordering of contained/contains EconomicResources. The last *indexed* resource will now be the first retrieved.
- Client modules:
	- Connection introspection logic for the GraphQL adapter updated for Holochain Beta RC (`@holochain/client` 0.11.9). **An admin websocket connection is now required in order to initialise the connection parameters.** We are hopeful this will be a temporary situation- see https://github.com/holochain/holochain/issues/1746
- Updates for compatibility with deprecated methods from Rust `chrono` crate
- Updated Nix environment configuration, which now requires Nix v2.4
- Various other maintenance / infrastructural updates for Holochain Beta RC

While the external API remains compatible, these changes are backwards-incompatible with the previous version's data structures and a reset & fresh installation of any DHTs is necessary to upgrade.

## hApp 0.1.0-beta

First stable API release, considered the starting point for integrations.

Compatible with the NPM modules [`@vf-ui/graphql-client-holochain`](https://www.npmjs.com/package/@vf-ui/graphql-client-holochain) and [`@valueflows/vf-graphql-holochain`](https://www.npmjs.com/package/@valueflows/vf-graphql-holochain) at version 0.0.1-alpha.13. Prior versions of these modules were published during the (unstable) alpha testing phase down to 0.0.1-alpha.1 and should be considered 'unstable'.

Previous unstable alpha versions of the hREA Holochain components also exist from 0.0.1-alpha.1 through to 0.0.1-alpha.7.

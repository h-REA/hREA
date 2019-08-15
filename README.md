# ValueFlows economic resource coordination software: Holochain implementation

A resource accounting framework and suite of apps for building economic coordination applications of any kind.

Implements a [Resource / Event / Agent (REA)](https://en.wikipedia.org/wiki/Resources,_events,_agents_(accounting_model)) network accounting system, based on the [ValueFlows](https://valueflo.ws/) protocol and built on [Holochain](https://holochain.org/).

<!-- MarkdownTOC -->

- [About](#about)
- [Repository structure](#repository-structure)
- [Setup](#setup)
- [Running](#running)
- [Developing](#developing)
- [Contributing](#contributing)
- [Docs](#docs)
- [License](#license)

<!-- /MarkdownTOC -->

## About

This Holochain application is actually a *suite of cooperating applications*, similar to 'microservices' in traditional client/server web architecture. Each app is packaged as a *'DNA'*- a concept specific to Holochain apps which essentially refers to a content-addressable network with its own application logic; in other words, Holochain apps are protocols defined as self-contained code bundles and are referenced by the hash of that code.

HoloREA is built to align with the [Open App Ecosystem](https://github.com/open-app/)'s ideologies and goals as well as that of the [Free Software Movement](https://www.gnu.org/philosophy/free-software-intro.en.html). Thus HoloREA's *'DNAs'* can be thought of as a framework for composing more complex end-user Holochain apps. For example, you might create a Holochain app to manage the internal logic of your cooperative or business and have it publish events out to separate HoloREA *'observation'* networks in order to share resources with distributors and suppliers. You might also choose to swap out HoloREA's *'agreements'* module with your own logic for managing agreements; or even combine HoloREA's own modules in nonstandard arrangements, like having multiple separate *'observation'* modules communicating with a shared *'planning'* space.

## Repository structure

- `example/` is filled with end-user applications built on the HoloREA framework.
	- `holorea-graphql-explorer/` is a [GraphiQL](https://github.com/graphql/graphiql) interface to the system with some added [additions to assist with comprehension](https://github.com/OneGraph/graphiql-explorer-example). Wired up to the development DNAs by default&mdash; super handy for testing and getting to know the ValueFlows [data structure](https://github.com/valueflows/vf-graphql/).
- [`happs/`](happs/README.md) contains subdirectories corresponding to separate Holochain app DNA packages, which expose their data as independent, isolated DHTs. Each DNA is composed of multiple *zomes* which describe semi-independent bits of functionality within that app DNA.
- `lib/` contains library code that is used by the various hApps (Holochain apps). Note that shared code is necessary to facilitate sharing of data between DHTs, as the de/serialisation logic is defined by Rust structs- hence this separation.
	- `hdk_graph_helpers/` is the result of abstracting away common functionality used in zome callback handlers to reduce boilerplate.
	- `vf_core/` contains core record structure types and type aliases for defining & linking records.
	- `vf_*/` the other folders in this directory correspond to the actual ValueFlows record structures and their behaviours, especially conversions.
- `modules/` is home to the JavaScript modules used in binding the Holochain backend to UIs, servers etc
	- `vf-graphql/` is the ValueFlows [reference spec](https://github.com/valueflows/vf-graphql/), used as the schema.
	- [`vf-graphql-holochain/`](modules/vf-graphql-holochain/README.md) contains the complete GraphQL schema adapter with bindings to Holochain DNA conductors (which expose the app DNAs defined in `happs/`).
- `test/` contains JavaScript integration tests for the system as a whole&mdash; they cover zome API tests and interactions between different app DHTs.
- `scripts/` just has utility script files used for DevOps tasks and configuring the repo.
- The NPM scripts in `package.json` at the top level of the repo are used to orchestrate test commands & run the apps for development. Please see the file for reference.

## Setup

1. Ensure you have all necessary [required software](./CONTRIBUTORS.md#required-software) installed. It is particularly important that you have [Nix](https://nixos.org) available in your `$PATH` if you wish to use the standard setup, otherwise advanced setup via Rustup and Cargo will be attempted.
2. Run `yarn` from this directory to bootstrap the repo. As part of the install script you will be loaded into the project's Nix shell. Simply exit the shell to finish installation.

Once configured, you should run `npm run shell` any time you're working on this project to bring all tooling online.

**Note that if you want your editor tooling to work as expected you will generally have to run it from within one of these shells, as well as all other CLI commands.** This excludes things like SSH which might need access to your homedir outside of the Nix sandbox. In other words:

- Run `npm run shell` before running any of the other NPM commands in this project's configuration.
- You may have issues running `git` and some other commands from within the Nix shell due to its reliance on privileged SSH configuration, but these can be run outside of Nix just fine.

## Running

Once installation has completed you can run `npm start` to boot up the following services:

- [GraphiQL query interface](example/holorea-graphql-explorer) backed by the [ValueFlows GraphQL spec](https://github.com/valueflows/vf-graphql/) at `http://localhost:3000`
- Holochain DNA HTTP interface at `http://localhost:4000`
- Holochain DNA websocket RPC interface at `ws://localhost:4001`
- TypeScript compiler daemon for rebuilding `vf-graphql-holochain` browser module upon changes

## Developing

You can also run `npm run dev`, which will boot up some listeners for triggering builds and re-running tests in response to code changes automatically.

For a complete list of available commands, see `package.json`'s scripts section.

## Contributing

For information on our workflow and contribution guidelines, see [CONTRIBUTORS.md](./CONTRIBUTORS.md).

Developers wishing to contribute should also refer to [recommended dev tools](./CONTRIBUTORS.md#recommended-dev-tools) for assistance in configuring your workstation for the most streamlined and pleasant development experience.

## Docs

See also the [wiki](https://github.com/holo-rea/ecosystem/wiki/Coordinating-the-REA-community-of-practise) for more information.

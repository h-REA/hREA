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
- [Known issues](#known-issues)
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

1. Ensure you have all necessary [required software installed](https://github.com/holo-rea/ecosystem/wiki/Setting-up-HoloREA-for-development#quick-start). It is particularly important that you have [Nix](https://nixos.org) available in your `$PATH` if you wish to use the standard setup, otherwise advanced setup via Rustup and Cargo will be attempted.
2. Run `nix-shell` in the root of this repo to boot and enter the Nix environment.
2. Run `yarn` from this directory to bootstrap the repo.

Once configured, you should run `nix-shell` any time you're working on this project to bring all tooling online.

## Running

Once installation has completed you can run `nix-shell` followed by `npm start` to boot up the following services:

- [GraphiQL query interface](example/holorea-graphql-explorer) backed by the [ValueFlows GraphQL spec](https://github.com/valueflows/vf-graphql/) at `http://localhost:3000`
- Holochain DNA HTTP interface at `http://localhost:4000`
- Holochain DNA websocket RPC interface at `ws://localhost:4001`
- TypeScript compiler daemon for rebuilding `vf-graphql-holochain` browser module upon changes

## Developing

Rather than `npm start` you can also run `npm run dev`, which will boot up some listeners for triggering builds and re-running tests in response to code changes automatically. To prevent the react-scripts dev server from hiding logs, you may want to run 3 separate terminals for the 3 daemon commands (`dht`, `ui` and `dev:graphql-adapter`) independently; and call `npm run build` and re-run tests manually as needed. Be sure to restart the `dht` command after any Rust compilation.

Running all integration tests in the `test` directory is accomplished with `npm run test:integration`. To run specific tests, use `npx tape test/**/*.js` substituting a path to an individual file.

To get logging output from the DNA code that is visible in tape tests, run any test commands with the environment variable `VERBOSE_DNA_DEBUG=1` set, and use the following Rust code:

```rust
hdk::debug(format!("{:?}", something));
```

A nice equivalent for JavaScript is:

```javascript
console.log(require('util').inspect(something, { depth: null, colors: true }))
```

For a complete list of available commands, see `package.json`'s scripts section.

If you plan on contributing to HoloREA's development, please read [the contributor guidelines](https://github.com/holo-rea/ecosystem/wiki/For-new-code-contributors) on the project wiki.

## Contributing

For information on our workflow and contribution guidelines, see [CONTRIBUTORS.md](./CONTRIBUTORS.md).

Developers wishing to contribute should also refer to [recommended dev tools](./CONTRIBUTORS.md#recommended-dev-tools) for assistance in configuring your workstation for the most streamlined and pleasant development experience.

## Known issues

- If you get `Bad owner or permissions on $HOME/.ssh/config` when attempting to use git remote commands or SSH from within the Nix shell, ensure your `~/.ssh/config` has `0644` permissions and not `0664`.

## Docs

See the [ecosystem wiki](https://github.com/holo-rea/ecosystem/wiki/) for more information.

## License

Licensed under an Apache 2.0 license.

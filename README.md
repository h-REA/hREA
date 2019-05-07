# ValueFlows economic resource coordination software: Holochain implementation

<!-- MarkdownTOC -->

- [Repository structure](#repository-structure)
- [Setup](#setup)
- [Contributing](#contributing)

<!-- /MarkdownTOC -->

## Repository structure

- `happs/` contains subdirectories corresponding to separate Holochain app DNA packages, which run as independent DHTs. These DNAs may contain multiple zomes but usually there is just one zome per DHT.
- `lib/` contains library code that is used by the various hApps (Holochain apps). Note that shared code is necessary to facilitate sharing of data between DHTs, as the de/serialisation logic is defined by Rust structs.
	- `vf-core/` is the ValueFlows system implemented in pure Rust. There are *no Holochain dependencies* in this module. All tests are implemented as standard Rust unit tests.
- `test/` contains JavaScript integration tests for the system as a whole&mdash; they cover zome API tests and interactions between different app DHTs.
- `scripts/` just has utility script files used for DevOps tasks and configuring the repo.
- Nodejs package commands in `package.json` at the top level of the repo are used to orchestrate test commands & run the apps for development.

## Setup

1. Ensure you have all necessary [required software](./CONTRIBUTORS.md#required-software) installed.
2. Run `yarn` from this directory to bootstrap the repo.


## Contributing

For information on our workflow and contribution guidelines, see [CONTRIBUTORS.md](./CONTRIBUTORS.md).

Developers wishing to contribute should also refer to [recommended dev tools](./CONTRIBUTORS.md#recommended-dev-tools) for assistance in configuring your workstation for the most streamlined and pleasant development experience.

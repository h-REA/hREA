# Developer documentation

<!-- MarkdownTOC -->

- [Quick start](#quick-start)
	- [Install required binaries](#install-required-binaries)
	- [Setup the project](#setup-the-project)
- [Understanding](#understanding)
- [Running](#running)
- [Contributing](#contributing)
	- [Environment variables](#environment-variables)
	- [Debugging](#debugging)
	- [Advanced execution](#advanced-execution)
	- [Recommended dev tools](#recommended-dev-tools)
		- [Linters](#linters)
		- [Editorconfig](#editorconfig)
		- [File headers](#file-headers)
	- [Known issues](#known-issues)
	- [Gotchas](#gotchas)
- [Publishing](#publishing)
	- [Publishing Node packages](#publishing-node-packages)
	- [Publishing a hApp Release](#publishing-a-happ-release)
- [Multi-project setup](#multi-project-setup)

<!-- /MarkdownTOC -->


## Quick start

(This is a short version of the [official Holochain install instructions](https://developer.holochain.org/start.html).)

### Install required binaries

You will need `holochain` & `hc`, `lair-keystore`, `cargo`, `node`, `pnpm` and `wasm-opt` installed and available on your path.

The easiest way to do this is using the built-in Nix shell. Simply [install Nix](https://nixos.org/download.html) and run `nix-shell` at the top level of this repository to load most of the necessary dependencies.

Here is a way to do this without nix-shell:
```
cargo install holochain --version 0.0.162 --locked
cargo install holochain_cli --version 0.0.57 --locked
cargo install lair_keystore --version 0.2.1
```

### Setup the project

1. Ensure you've loaded the project Nix env, if in doubt run `nix-shell` from the repository root.
2. `pnpm i` to install node packages
3. `npm run build` to compile. You'll see some TypeScript errors when building the GraphQL client which can safely be ignored.

## Understanding

To get an initial grasp of the project, at an overview level, check out the separate [repository structure](./repository-structure.md) document.

## Running

An `npm start` will boot up all development services needed to rebuild the project in realtime. The scripts in `package.json` are self-documenting and can be used as a reference if you wish to run more fine-grained commands.

- [GraphiQL query interface](apps/hrea-graphql-explorer) backed by the [ValueFlows GraphQL spec](https://github.com/valueflows/vf-graphql/) at `http://localhost:3000`
- Holochain app websocket RPC interface at `ws://localhost:4000`
- Holochain admin websocket RPC interface running on a random port, controlled via `hc s call`
- TypeScript compiler daemon for rebuilding `vf-graphql-holochain` browser module upon changes

:TODO: integrate realtime rebuilding of Rust crates





## Contributing

If you are interested in contributing to hREA's development we delightedly review all pull requests. For more engaged contributors wishing to make long-term contributions we have a lightweight coordination workflow we practise together.

- [Contributor workflow](Contributor-workflow.md) (contribution protocol, git best practises & coding standards)
- "[For new code contributors](https://github.com/h-REA/ecosystem/wiki/For-new-code-contributors)" on the project ecosystem wiki has further information on how to engage with the project.

For other details related to interacting with this codebase at a technical level, read on.

### Environment variables

Scripts in this repository respond to the following env vars:

Execution parameters:

- `HOLOCHAIN_APP_PORT` sets the websocket port for the app interface when running the conductor in a development sandbox. See the `dht:conductor` script in `package.json`.
- `HOLOCHAIN_DNA_UTIL_PATH` works similarly to `TRYORAMA_HOLOCHAIN_PATH`, but for the `hc` binary that ships with Holochain. It is called to finalise packaging the bundles in `bundles/` and to run the dev environment conductor.

Build parameters:

- `RUN_WASM_OPT=0` to disable the WASM optimisation pass during development, as it can be slow and CPU-intensive.

Test parameters:

- `GRAPHQL_DEBUG=1` will enable debug output for the parameters transmitted and received by the GraphQL connection used in tests.
- `WASM_LOG=debug` `RUST_LOG="debug,wasmer_compiler_cranelift=error,holochain::core::workflow=error"` `RUST_BACKTRACE=1` are all set when executing the integration test suite.
- `TRYORAMA_LOG_LEVEL=debug` is ALSO required in order to view the logs output from your WASM code


### Debugging

Most of the time during development, you won't want to run the whole test suite but rather just those tests you're currently working on. The usual workflow when developing a module in isolation is:

1. `npm run build:holochain:dev` from the repository root to rebuild the module(s) you are working on.
2. `TRYORAMA_LOG_LEVEL=debug WASM_LOG=debug RUST_LOG="debug,wasmer_compiler_cranelift=error,holochain::core::workflow=error" RUST_BACKTRACE=1 npx tape test/**/*.js` from the `test` directory to run specific tests, substituting a path to an individual file. Note the [env vars](#environment-variables) used here are needed to obtain debug output from the zome code.

Getting debug output printed to the screen depends on where you are logging from.

- In your Rust code, prefix any debug logging with some format string, or use named arguments-
	```rust
	debug!("WARGH {:?}", something);
	debug!(named = somethings, work = "too");
	```
- In JavaScript code, using `console.error` instead of `console.log` will make the output visible, even when piping test output into `faucet` to reduce verbosity. You might also want to get more depth in your output than the built-in serializers provide, especially when interacting with GraphQL result objects-
	```javascript
	console.error(require('util').inspect(something, { depth: null, colors: true }))
	```

Debug output from the Holochain conductor can be noisy, which is why all test scripts coded in `package.json` pipe the test output to [tap-dot](https://github.com/scottcorgan/tap-dot)). Remember that you can always add nonsense strings to your debug output and pipe things into `| grep 'XXXX'` instead of `| npx tap-dot` if you need to locate something specific and the text is overwhelming.

Another way to reduce noice from the Holochain conductor logs, if you want to go down to debug level logs, is to use a config
var like the following for RUST_LOG, which `holochain` will respect:
```
RUST_LOG="debug,wasmer_compiler_cranelift=error,holochain::core::workflow=error"
```
You can [learn more here](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html).

### Advanced execution

If you look at the commands in `package.json` you will see that they are namespaced into groups of functionality. You can also see which commands depend on each other. Most of the time it will be more efficient to understand the command structure and run individual commands than it will be to boot the whole system together.

Something you may find painful when debugging is that the `react-scripts` Webpack configuration used by some UI apps clears the terminal when it is active. To work around this, you can run these commands in separate terminals so that the output is not truncated. Running the system like this would be a case of:

- Running `npm run build` first
- `npm run dht` in a separate terminal to boot the Holochain conductor
- `npm run dev:graphql:adapter` in its own terminal if you plan on editing the GraphQL code & want realtime feedback on your changes
- `npm run dev:graphql:explorer` to boot up the GraphiQL app UI to interact with the DNAs, or boot any other UI apps instead



### Recommended dev tools

#### Linters

For Rust, install [Clippy]. `rustup component add clippy` is executed after setting up the repo, so you should not need to do anything other than setup Rust for your editor:

- **Sublime Text:**
	- `Rust Enhanced` and `SublimeLinter-contrib-rustc` via Package Control will give you autocomplete and error output upon saving files.
- **VSCode:**
	- Install the `Rust (rls)` extension via the marketplace
	- Set `rust-client.disableRustup = false` in the editor configuration (Rust versions are managed by Nix)
	- For advanced users you can also setup a language server to get realtime code hinting & errors as you type, [for more info, see here](https://hoverbear.org/2017/03/03/setting-up-a-rust-devenv/).

For JavaScript, install [eslint]. All necessary dependencies should be installed via NPM upon initialising the repository, but you must still configure your editor to show linter output:

- **Sublime Text:**
		- `SublimeLinter-eslint` and `SublimeLinter-tslint` are both used, depending on whether editing JS or TS files.
- **VSCode:**
		- Install the `ESLint` extension via the marketplace.

#### Editorconfig

This ensures consistency in file formatting. Install a plugin for your editor according to the following:

- **Sublime Text:**
	- `EditorConfig` via Package Control
- **VSCode:**
	- `EditorConfig for VSCode` via the marketplace

#### File headers

You can configure your editor to automatically add new header comment blocks to files you create.

- **Sublime Text:**
	- Install `FileHeader` via Package Control
	- Go to *Preferences > Package Settings > FileHeader > Settings - User* to locate your `custom_template_header_path`
	- Also add this block to your settings:
		```
		"Default": {
			"author": "YOURNAME",
			"email": "YOURNAME@example.com"
		},
		```
	- *(Note this configuration can also be specified on a per-project basis under `settings.FileHeader` in your project config JSON file.)*
	- Edit files in this folder to set the content to prepend to new files you create.
- **VSCode:**
	- *:TODO:*


### Known issues

- The Visual Studio Code terminal can cause issues with Nix, especially on Windows. Use a standalone terminal instead of the one built in to the editor avoid potential problems.
- If you get `Bad owner or permissions on $HOME/.ssh/config` when attempting to use git remote commands or SSH from within the Nix shell, ensure your `~/.ssh/config` has `0644` permissions and not `0664`.
- When loading your Nix environment you may encounter the error `unknown compression method 'zstd'`. If this occurs, ensure you are running Nix v2.4 or above. *(See [upgrading Nix](https://nixos.org/manual/nix/unstable/installation/upgrading.html).)*

### Gotchas

- Inconsistent state behaviours in tests:
	- This is most often due to mis-use of `await s.consistency()` in test code. Ensure that consistency checks are *only* present after `mutation` GraphQL operations and JSONRPC calls which modify the source-chain state; i.e. after a GraphQL `query` one should *not* perform a consistency wait.
- Receiving incorrect record IDs when retrieving records:
	- These errors are often encountered when confusing cross-DNA link fields for same-DNA links. Check that you are using the appropriate helpers for the link type (`_index` vs `_remote_index` helpers).


## Publishing

### Publishing Node packages

The JavaScript API client modules are published to NPM with PNPM. **You must use PNPM** to publish these, since packages contain PNPM-specific workspace metadata that NPM does not know how to deal with.

- Ensure all packages requiring publication have their `version` field in `package.json` updated to reflect the next version to be published.
- Ensure a successful `pnpm run build` completes after the version updates are made.
- Run `pnpm -r publish --access public` from the root directory to publish all packages with new versions.

### Publishing a hApp Release

Publishing a hApp release is actually easy, thanks to the [Github Actions automation](https://github.com/h-REA/hREA/blob/sprout/.github/workflows/release.yml). 

The workflow will automatically create a release on Github, if you tag a commit locally using a correctly formatted string pattern, and push it to github. The string pattern should look similar to `happ-0.0.1-alpha.7`. It could be as minimal as `happ-0.0.1`, but we are still publishing with the `alpha` label for now.

To do this:
1. make sure you are on the intended git commit and have no uncommitted changes
2. make sure your commits are all pushed: `git push`
3. perform the git tag: `git tag happ-0.0.1-alpha.7`
4. push the git tag: `git push --tags`
5. Go to the [Github Actions tab](https://github.com/h-REA/hREA/actions) and find the latest running workflow named `Release`. You can track the progress.
6. Once the build has finished, check the release, and provide release notes and changelog details.
7. That's it!


TODO: instructions for publishing Rust crates

TODO: instructions for publishing built DNA & zome artifacts to Holochain Devhub


## Multi-project setup

For developers who need to work on other ValueFlows-related codebases whilst developing hREA, check out the [ValueFlows project metarepo](https://github.com/h-REA/valueflows-project-metarepo/).

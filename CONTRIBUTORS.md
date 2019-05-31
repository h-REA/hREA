# Contribution guidelines


<!-- MarkdownTOC -->

- [Required software](#required-software)
- [Recommended dev tools](#recommended-dev-tools)
	- [Code quality](#code-quality)
		- [Linters](#linters)
		- [Editorconfig](#editorconfig)
		- [File headers](#file-headers)
	- [Version locking](#version-locking)
- [Git conventions](#git-conventions)
	- [Best practises](#best-practises)
	- [Branching workflow](#branching-workflow)
	- [Release management](#release-management)
- [Codebase-specific instructions](#codebase-specific-instructions)

<!-- /MarkdownTOC -->


## Required software

(This is a short version of the [official Holochain install instructions](https://developer.holochain.org/start.html)

**Nodejs**

1. For development, it is highly recommended to [install NVM](https://github.com/creationix/nvm) to manage nodejs versions. Once installed:

```
nvm install $(cat .nvmrc)
```

Or if you wish to do it manually, ensure the version of node you're using corresponds with that indicated in the `.nvmrc` file.

2. Once nodejs is setup, install Yarn if you don't already have it: `npm i -g yarn`.

**Rust**

For development, it is highly recommended to install via RustUp:

```
curl https://sh.rustup.rs -sSf
source $HOME/.cargo/env
rustup toolchain install nightly-2019-02-04
rustup default nightly-2019-02-04	# optional
rustup target add wasm32-unknown-unknown --toolchain nightly-2019-02-04
```

We also recommend to set a default toolchain override for this directory when cloning. This is done automatically when running NPM setup- see `scripts/postinstall.sh` for details.

**Other dependencies**

- You need the `libssl` development packages installed to compile Holochain.
	- For Ubuntu users: `sudo apt install libssl-dev`

**`hc` CLI and `holochain` runtime**

The Holochain toolchain will be installed for you at a known working version when initialising this repo. If  you see errors like *"binary `hc` already exists"* upon installing or you wish to install yourself, you can do so with the following command, substituting `branch`, `tag` or `ref` to target a specific version from git; and `hc` or `holochain` depending on the pre-existing binary in conflict. Note that you must have Rust and libssl-dev installed before proceeding to this step.

```
cargo install hc --force --git https://github.com/holochain/holochain-rust.git --branch develop
cargo install holochain --force --git https://github.com/holochain/holochain-rust.git --branch develop
```


## Recommended dev tools

### Code quality

#### Linters

For Rust, install [Clippy]. `rustup component add clippy` is executed after setting up the repo, so you should not need to do anything other than setup Rust for your editor:

- **Sublime Text:**
	- `Rust Enhanced` and `SublimeLinter-contrib-rustc` via Package Control will give you autocomplete and error output upon saving files.
- **VSCode:**
	- `Rust` extension via the marketplace
	- For advanced users you can also setup a language server to get realtime code hinting & errors as you type, [for more info, see here](https://hoverbear.org/2017/03/03/setting-up-a-rust-devenv/).

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
			"email": "YOURNAME@consensys.net"
		},
	  ```
	- *(Note this configuration can also be specified on a per-project basis under `settings.FileHeader` in your project config JSON file.)*
	- Edit files in this folder to set the content to prepend to new files you create.
- **VSCode:**
	- *:TODO:*

For a description of the Rust documentation comment conventions, see [this manual section](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#commonly-used-sections).

### Version locking

This project uses [`.nvmrc` files](https://github.com/creationix/nvm#nvmrc) to specify the correct nodejs versions to run when developing. You can install some additional shell hooks into [zsh on OSX](https://github.com/creationix/nvm#zsh) or place this in your `.bashrc` on Linux to auto-switch to the correct node version as you move around:

```
cd () { builtin cd "$@" && chNodeVersion; }
pushd () { builtin pushd "$@" && chNodeVersion; }
popd () { builtin popd "$@" && chNodeVersion; }
chNodeVersion() {
    if [ -f ".nvmrc" ] ; then
        nvm use;
    fi
}
chNodeVersion;
```

To manage Rust versions, we presume that you are using [RustUp](https://rustup.rs/). With this under consideration, there is a toolchain override for the project pre-configured in `scripts/postinstall.sh`.






## Git conventions


### Best practises

- Commit messages should take the imperative form; ie. finish the sentence *"Applying this commit will [...]"*
- Use descriptive, single-line commit messages
- Make commits as atomic as possible. This makes life easier for a [variety of reasons](https://brainlessdeveloper.com/2018/02/19/git-best-practices-atomic-commits/). Heavily consider using `git add -p`, and *never* use `git add -A`.


### Branching workflow

We use a [gitflow](https://danielkummer.github.io/git-flow-cheatsheet/)-inspired but slightly less onerous process:

- We work off `master` and consider it our stable integration branch
- All work on improving the systems occurs in branches prefixed with `feature/XX-`, where `XX` is the related Github issue number
	- Feature branches are merged back to `master` *only* when fully integrated and tested
	- It is expected that developers working on new features merge in updates from `master` as work unfolds in order to minimise merge conflicts later
	- It is fine to merge work from another feature branch into your own in the case of interdependent features which cannot be completed without integrating
- Miscellaneous patches which don't fall under the usual improvement workflow can be undertaken in `hotfix/XX-` branches and merged back to master when tested and ready for deployment
- When finishing up work in any branch, it should be deleted and removed from Github.


### Release management

- A `release/` branch is opened to commit updates to package version numbers, README files and any other release prep work (eg. building documentation)
- Once ready, `release/` is merged to `master`.
- The resultant commit is tagged using semver with no prefix, eg. `0.13.2`.






## Codebase-specific instructions


### Creating new DNAs

1. `cd happs/`
2. `hc init <NEW_DNA_NAME>` scaffolds a new DNA folder named `NEW_DNA_NAME`.
3. Edit `app.json` in the newly created folder as appropriate.
4. Remove these generated files from the newly created directory:
	- `test/` (integration tests are all contained in the top-level `test` directory)
	- `.gitignore` (already taken care of via project-global ignore file)
5. Wire up a new `build` sub-command in the toplevel `package.json`; eg. `"build:dna_obs": "cd happs/observation && hc package"`. Do not forget to add the new build step to the base NPM `build` script.
6. Edit `conductor-config.toml` as appropriate to include instance configuration & bridging for any new DHTs to be loaded from this DNA in the local test environment.


### Creating new zomes

1. From the root directory, run `hc generate happs/DNA_NAME/zomes/ZOME_NAME`, substituting the name of the DNA you're targeting and zome you're creating. Or you can `cd` to the folder yourself&mdash; the argument to `hc generate` is just a path.
2. Add the new zome path to the `members` section of `Cargo.toml` workspace in *this* directory.
3. Edit `code/Cargo.toml` in the new zome folder:
	- Give the package a nice name; and **ensure the same name is present in `.hcbuild`**.
	- Add the comment `# :DUPE: hdk-rust-revid` above the Holochain dependencies so that these version tags can be easily located later.
	- Add any dependencies to shared library code using path references, eg. `vf_core = { path = "../../../../../lib/vf_core" }`
4. Set the description in `zome.json`


### Updating the Holochain platform

**If you have the Holochain Rust crates installed locally (not via NIX):**

1. Ensure you are using the correct Rust toolchain for this repo. If you have previously configured the repository, this should be fine. If you are unsure, check that the output from `rustup show` matches the *'rustup override set'* line in `scripts/postinstall.sh`.
2. Run the following, substituting the git tag of the version you are updating to for `$NEWTAG`:
  `cargo install holochain hc --git https://github.com/holochain/holochain-rust.git --tag $NEWTAG --force`
3. Change `HDK_RUST_REVID` in `scripts/postinstall.sh` to match the version you have updated to so that new contributors have their tooling configured properly.
4. Locate all other references to the old Holochain dependency versions in `Cargo.toml` files and update to the new version. All instances should be locateable by searching the codebase for the string `:DUPE: hdk-rust-revid`.
5. Track down the appropriate version of the `holochain-nodejs` module used in integration tests by referencing the version of Holochain you are using against `https://github.com/holochain/holochain-rust/blob/${YOUR_VERSION_TAG}/nodejs_conductor/package.json`. Set this as the appropriate package version for `@holochain/holochain-nodejs` in `test/package.json`.

**:TODO: instructions for NIX users**

# Contribution guidelines


<!-- MarkdownTOC -->

- [Required software](#required-software)
	- [Nix](#nix)
- [Recommended dev tools](#recommended-dev-tools)
	- [Code quality](#code-quality)
		- [Linters](#linters)
		- [Editorconfig](#editorconfig)
		- [File headers](#file-headers)
	- [Version locking](#version-locking)
- [Git conventions](#git-conventions)
	- [Best practises](#best-practises)
	- [Branching workflow](#branching-workflow)
- [Contributor workflow & coordination protocol](#contributor-workflow--coordination-protocol)
	- [Tracking tasks](#tracking-tasks)
	- [Picking up new work](#picking-up-new-work)
	- [Completing finished work](#completing-finished-work)
	- [Release management](#release-management)
- [Codebase-specific instructions](#codebase-specific-instructions)
	- [Creating new DNAs](#creating-new-dnas)
	- [Creating new zomes](#creating-new-zomes)
	- [Updating the Holochain platform](#updating-the-holochain-platform)

<!-- /MarkdownTOC -->


## Required software

(This is a short version of the [official Holochain install instructions](https://developer.holochain.org/start.html)

### Nix

You need to run your Holochain tooling (`hc` & `holochain` binaries, `cargo`, `rustc`, `node` etc **and your editor**) from within a Nix shell in order to have access to all the CLI applications you'll need in development. It is installed via:

	curl https://nixos.org/nix/install | sh

You should now have `nix-shell` available in your PATH and be able to proceed with running this package's installation steps.

**Linux users:** if you get warnings about `libgtk3-nocsd.so.0`, you should add this line to your `~/.profile` (or other) file before the `nix.sh` line:

	export LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libgtk3-nocsd.so.0




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


## Contributor workflow & coordination protocol


### Tracking tasks

- All work is logged as standard Github issues. We mostly use the labels `enhancement`, `bug`, `question`, `decision` & `user story`.
- Issues are grouped into Github milestones. Milestones describe major features and have no particular ordering or relationship to each other, unless otherwise indicated in their descriptions.

### Picking up new work

- **Assigning oneself to an issue indicates a commitment to completing the task.** Before picking up an issue with an existing assignee, one should check with the other person(s) for a handover.
- Before beginning a task, one should coordinate with other developers who may depend on the outcome. Contributors should follow a [design by contract](https://en.wikipedia.org/wiki/Design_by_contract) approach and decide on interfaces for common code together before proceeding with implementation. This conversation is best had in the Github issue thread.
- If working within the `holo-rea` codebase, branches should be named in the format `feature/XX-some-issue` where `XX` is the Github issue ID of the primary task related to the work. If working in one's own fork of the repository any branch names are fine, but it is recommended to provide links to your private branch in the issue comments thread so that others may follow along.

### Completing finished work

- All new contributions must come with full test coverage to prove that the requested features are provided. **No pull requests should be submitted without including tests.**
- Completed work should be submitted as a Github pull request. Another project contributor must approve all pull requests in addition to the author.
	- *(In the event that only a single contributor is maintaining the project, one may merge their own pull requests **provided** that full test coverage is also included.)*
- Once PRs have been approved they are merged to `master` and the feature branch is deleted.

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

The instructions apply to the officially supported Nix release of Holochain. An upgrade for natively installed Rust packages is much the same, except for use of `cargo install` commands instead of the Nix-specific steps.

1. Upgrade Holonix according to the "how to upgrade holonix" section on https://docs.holochain.love/docs/configure/
	- Change `ref` in `config.nix` to the latest Holonix release tag on Github and alter `sha256` to invalidate its cache
	- Attempt to drop into the nix shell, it will error with “hash mismatch”
	- Copy the “got:” hash for the new ref to `holonix.github.sha256`
2. Run `nix-shell` to boot into the Nix environment
3. `npm run clean:build` from the root directory to wipe Rust build files and refresh the cargo cache to match the new HC version.
4. Change `HDK_RUST_REVID` in `scripts/postinstall.sh` to match the version you have updated to so that new contributors have their tooling configured properly. The appropriate HDK revision ID can be found in `dist/config.nix` in the Holonix repository.
5. Locate all other references to the old Holochain dependency versions in `Cargo.toml` files and update to the new `HDK_RUST_REVID` version. All instances should be locateable by searching the codebase for the string `:DUPE: hdk-rust-revid`.
6. Ensure the latest available version of [Diorama](https://www.npmjs.com/package/@holochain/diorama) is also configured in `test/package.json`.

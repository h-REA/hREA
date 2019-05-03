# HoloREA DHT logic

<!-- MarkdownTOC -->

- [How-to](#how-to)
	- [Creating new zomes](#creating-new-zomes)

<!-- /MarkdownTOC -->

## How-to

### Creating new zomes

1. Run `hc generate zomes/XXXX`, substituting the name of the zome you're creating.
2. Add the new zome path to the `members` section of `Cargo.toml` in *this* directory.
3. Edit `code/Cargo.toml` in the new zome folder, replacing `branch = "develop"` with `rev = "XXXXXXX"`, substituting the locked Holochain revision ID indicated in `scripts/postinstall.sh`. Also add the following comment above that line: `# :DUPE: hdk-rust-revid`.

---
to: lib/<%= h.changeCase.snake(zome_name) %>/defs/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>_defs"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]
serde = "1"
# :DUPE: hdk-rust-revid
hdk3 = {git = "https://github.com/holochain/holochain", rev = "ed0d4e8a8", package = "hdk3"}

hc_zome_<%= h.changeCase.snake(zome_name) %>_storage = { path = "../storage" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_storage_consts = { path = "../storage_consts" }
hc_zome_TODO_storage_consts = { path = "../../XXX/storage_consts" }

[lib]
crate-type = ["lib"]

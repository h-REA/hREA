---
to: lib/<%= h.changeCase.snake(zome_name) %>/lib/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>_lib"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]
# :DUPE: hdk-rust-revid
hdk3 = {git = "https://github.com/holochain/holochain", rev = "ea8d62a4c", package = "hdk3"}

hdk_graph_helpers = { path = "../../hdk_graph_helpers" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_storage_consts = { path = "../storage_consts" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_storage = { path = "../storage" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc = { path = "../rpc" }
hc_zome_TODO_storage_consts = { path = "../../TODO/storage_consts" }

[lib]
crate-type = ["lib"]

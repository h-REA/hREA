---
to: lib/<%= h.changeCase.snake(zome_name) %>/storage/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>_storage"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]
serde = "1.0.104"
serde_json = { version = "1.0.47", features = ["preserve_order"] }
serde_derive = "1.0.104"
# :DUPE: hdk-rust-revid
holochain_json_api = "0.0.23"
holochain_json_derive = "0.0.23"

hdk_graph_helpers = { path = "../../hdk_graph_helpers" }
vf_core = { path = "../../vf_core" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc = { path = "../rpc" }

[lib]
crate-type = ["lib"]

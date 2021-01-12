---
to: <%=dna_path%>/zomes/<%= h.changeCase.snake(zome_name) %>/code/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]
serde = "1"
# :DUPE: hdk-rust-revid
hdk3 = {git = "https://github.com/holochain/holochain", rev = "ea8d62a4c", package = "hdk3"}

hc_zome_<%= h.changeCase.snake(zome_name) %>_defs = { path = "../../../../../lib/<%= h.changeCase.snake(zome_name) %>/defs" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc = { path = "../../../../../lib/<%= h.changeCase.snake(zome_name) %>/rpc" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_lib = { path = "../../../../../lib/<%= h.changeCase.snake(zome_name) %>/lib" }

[lib]
path = "src/lib.rs"
crate-type = ["cdylib"]

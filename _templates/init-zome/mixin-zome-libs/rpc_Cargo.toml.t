---
to: lib/<%= h.changeCase.snake(zome_name) %>/rpc/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]
serde = "1"

hdk_records = { path = "../../hdk_records" }
vf_attributes_hdk = { path = "../../vf_attributes_hdk" }

[lib]
crate-type = ["lib"]

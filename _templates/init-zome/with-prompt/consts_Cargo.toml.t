---
to: lib/<%= h.changeCase.snake(zome_name) %>/storage_consts/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>_storage_consts"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]

[lib]
crate-type = ["lib"]

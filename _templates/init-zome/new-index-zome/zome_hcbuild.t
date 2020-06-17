---
to: <%=dna_path%>/zomes/<%= h.changeCase.snake(foreign_record_name) %>_idx/code/.hcbuild
---
{
  "steps": [
    {
      "command": "cargo",
      "arguments": [
        "build",
        "--release",
        "--target=wasm32-unknown-unknown",
        "--target-dir=/tmp/holochain/target"
      ]
    }
  ],
  "artifact": "/tmp/holochain/target/wasm32-unknown-unknown/release/hc_zome_<%= h.changeCase.snake(foreign_record_name) %>_index_<%= h.changeCase.snake(local_dna_name) %>.wasm"
}

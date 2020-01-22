---
to: <%=dna_path%>/zomes/<%= h.changeCase.snake(zome_name) %>/code/.hcbuild
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
  "artifact": "/tmp/holochain/target/wasm32-unknown-unknown/release/hc_zome_<%= h.changeCase.snake(zome_name) %>.wasm"
}

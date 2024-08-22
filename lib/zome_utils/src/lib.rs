use hdk::prelude::*;
pub fn link_input(
  base_address: impl Into<AnyLinkableHash>,
  link_type: impl LinkTypeFilterExt,
  tag: Option<LinkTag>) -> GetLinksInput {
  let mut input = GetLinksInputBuilder::try_new(base_address, link_type).unwrap();
  if let Some(taggy) = tag {
     input = input.tag_prefix(taggy);
  }
  input.build()
}
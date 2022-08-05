use hdi::prelude::*;

#[hdk_link_types]
pub enum LinkTypes {
    SemanticIndex,
    TimeIndex,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryType)]
pub enum EntryTypes {
    IndexSegment(IndexSegment),
}

// does this need an entry def id of "time_index"
#[hdk_entry_helper]
#[derive(Clone)]
pub struct IndexSegment(u64);
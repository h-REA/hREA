/**
 * Type aliases used to ensure explicit awareness of applicable record types in VF structs
 *
 * Note that all types are defined as optional by default, since most fields in VF are optional.
 */

use hdk::holochain_core_types::{
    cas::content::Address,
    time::Iso8601,
};

pub type Timestamp = Option<Iso8601>;

pub type ExternalURL = Option<String>;

pub type LocationAddress = Option<Address>;

pub type UnitAddress = Option<Address>;

pub type AgentAddress = Option<Address>;

pub type ResourceAddress = Option<Address>;
pub type ProcessOrTransferAddress = Option<Address>;

pub type ResourceSpecificationAddress = Option<Address>;

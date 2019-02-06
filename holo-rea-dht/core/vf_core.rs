/**
 * Core types for the ValueFlows system
 *
 * @package: HoloREA
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-02-06
 */

use serde_derive::{ Serialize, Deserialize };
use holochain_core_types_derive::{ DefaultJson };
use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};

/**
 * VfEntry is the base class for entities that have to do with VF.
 * The standard says that there are a few fields that any object could have.
 */
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct VfEntry {
  name: Option<String>,
  image: Option<String>,
  note: Option<String>,
  url: Option<String>,
}

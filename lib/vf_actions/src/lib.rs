/**
 * ValueFlows-compatible action type definitions and records
 *
 * @package Holo-REA
 * @since
 */
use serde::{Deserializer, Serializer, de::Error};

use hdk::prelude::*;
use vf_attributes_hdk::{ ActionId, ProcessAddress, EconomicResourceAddress };

pub mod builtins;
pub use builtins::{ get_builtin_action, get_all_builtin_actions };

#[derive(SerializedBytes, Debug, Clone, Copy, PartialEq)]
pub enum ActionEffect {
    // for 'process' events
    NoEffect,
    Increment,
    Decrement,
    // for 'transfer' events
    DecrementIncrement,
}

impl Serialize for ActionEffect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            ActionEffect::NoEffect => "noEffect",
            ActionEffect::Increment => "increment",
            ActionEffect::Decrement => "decrement",
            ActionEffect::DecrementIncrement => "decrementIncrement",
        })
    }
}

impl<'de> Deserialize<'de> for ActionEffect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "noEffect" => ActionEffect::NoEffect,
            "increment" => ActionEffect::Increment,
            "decrement" => ActionEffect::Decrement,
            "decrementIncrement" => ActionEffect::DecrementIncrement,
            &_ => Err(
                D::Error::custom(format!("Invalid value for ActionEffect: {}", s.as_str()))
            )?,
        })
    }
}

// actual underlying operations applied to particular resources are a subset of higher-level ActionEffect
#[derive(Debug)]
pub enum ActionInventoryEffect {
    NoEffect,
    Increment,
    Decrement,
}

#[derive(SerializedBytes, Debug, Clone, Copy, PartialEq)]
pub enum ProcessType {
    NotApplicable,
    Input,
    Output,
}

impl Serialize for ProcessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            ProcessType::NotApplicable => "notApplicable",
            ProcessType::Input => "input",
            ProcessType::Output => "output",
        })
    }
}

impl<'de> Deserialize<'de> for ProcessType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "notApplicable" => ProcessType::NotApplicable,
            "input" => ProcessType::Input,
            "output" => ProcessType::Output,
            &_ => Err(
                D::Error::custom(format!("Invalid value for ProcessType: {}", s.as_str()))
            )?,
        })
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub id: String,
    pub label: String,
    pub resource_effect: ActionEffect,
    pub input_output: ProcessType,
    pub pairs_with: String, // any of the action labels, or "notApplicable"
}

/**
 * Validation for EconomicEvent, Commitment and Process to ensure correct use of actions & Processes
 */
pub fn validate_flow_action(action_id: ActionId, input_process: Option<ProcessAddress>, output_process: Option<ProcessAddress>) -> Result<(), String> {
    if let Some(action) = get_builtin_action(action_id.as_ref()) {
        match action.input_output {
            ProcessType::NotApplicable => if input_process.is_some() || output_process.is_some() {
                Err(format!("EconomicEvent of '{:}' action cannot link to processes", action.id).into())
            } else { Ok(()) },
            ProcessType::Input => if input_process.is_none() {
                Err(format!("EconomicEvent input process required for '{:}' action", action.id).into())
            } else { Ok(()) },
            ProcessType::Output => if output_process.is_none() {
                Err(format!("EconomicEvent output process required for '{:}' action", action.id).into())
            } else { Ok(()) },
        }
    } else {
        Err("Unknown action".to_string())
    }
}

pub fn validate_move_inventories(resouce_inventoried_as: Option<EconomicResourceAddress>, to_resource_inventoried_as: Option<EconomicResourceAddress>) -> Result<(), String> {
    match resouce_inventoried_as {
        Some(_) => match to_resource_inventoried_as {
            Some(_) => Ok(()),
            None => Err("inventoried move EconomicEvent requires both source and destination inventory fields".into()),
        },
        None => match to_resource_inventoried_as {
            None => Ok(()),
            Some(_) => Err("non-inventoried move EconomicEvent must omit inventory fields".into()),
        },
    }
}

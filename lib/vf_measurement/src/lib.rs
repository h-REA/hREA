/**
 * Structures for encapsulating semantic meaning of measurements
 *
 * @package     hREA
 * @since       2019-05-09
 */
use vf_attributes_hdk::UnitId;
use hdk_records::{RecordAPIResult, DataIntegrityError};
use hdk::prelude::*;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: UnitId,
    pub name: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuantityValue {
    // :TODO: https://users.rust-lang.org/t/currency-in-rust/890/9 ?
    has_numerical_value: f64,     // :NOTE: uses https://en.wikipedia.org/wiki/IEEE_754 for math
    #[serde(default)]
    has_unit: Option<UnitId>,
}

impl<'a> QuantityValue {
    pub fn new(has_numerical_value: f64, has_unit: Option<UnitId>) -> QuantityValue {
        QuantityValue {
            has_numerical_value,
            has_unit,
        }
    }

    pub fn get_numerical_value(&'a self) -> f64 {
        self.has_numerical_value.to_owned()
    }

    pub fn get_unit(&'a self) -> Option<UnitId> {
        self.has_unit.to_owned()
    }
}

pub fn add(q1: QuantityValue, q2: QuantityValue) -> RecordAPIResult<QuantityValue> {
    if q1.has_unit != q2.has_unit {
        return Err(DataIntegrityError::MismatchingUnits(q1.get_unit().map(|unit| unit.1), q2.get_unit().map(|unit| unit.1)));
    }
    Ok(
        QuantityValue {
            has_numerical_value: q1.has_numerical_value + q2.has_numerical_value,
            has_unit: q1.has_unit,
        }
    )
}

pub fn subtract(q1: QuantityValue, q2: QuantityValue) -> RecordAPIResult<QuantityValue> {
    if q1.has_unit != q2.has_unit {
        return Err(DataIntegrityError::MismatchingUnits(q1.get_unit().map(|unit| unit.1), q2.get_unit().map(|unit| unit.1)));
    }
    Ok(
        QuantityValue {
            has_numerical_value: q1.has_numerical_value - q2.has_numerical_value,
            has_unit: q1.has_unit,
        }
    )
}

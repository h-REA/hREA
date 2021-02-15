/**
 * Structures for encapsulating semantic meaning of measurements
 *
 * @package     Holo-REA
 * @since       2019-05-09
 */
use holochain_serialized_bytes::prelude::*;
use super::type_aliases::UnitId;

#[derive(Debug, Clone)]
pub struct Unit {
    id: UnitId,
    name: Option<String>,
    symbol: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
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

    pub fn get_unit(&'a self) -> Option<UnitId> {
        self.has_unit.to_owned()
    }
}

pub fn add(q1: QuantityValue, q2: QuantityValue) -> QuantityValue {
    if q1.has_unit != q2.has_unit {
        panic!("Unimplemented! Need to enable unit conversions in QuantityValue math");
    }
    QuantityValue {
        has_numerical_value: q1.has_numerical_value + q2.has_numerical_value,
        has_unit: q1.has_unit,
    }
}

pub fn subtract(q1: QuantityValue, q2: QuantityValue) -> QuantityValue {
    if q1.has_unit != q2.has_unit {
        panic!("Unimplemented! Need to enable unit conversions in QuantityValue math");
    }
    QuantityValue {
        has_numerical_value: q1.has_numerical_value - q2.has_numerical_value,
        has_unit: q1.has_unit,
    }
}

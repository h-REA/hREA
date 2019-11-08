use super::type_aliases::UnitAddress;

#[derive(Debug, Clone)]
pub struct Unit {
    id: UnitAddress,
    name: Option<String>,
    symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QuantityValue {
    // :TODO: https://users.rust-lang.org/t/currency-in-rust/890/9 ?
    numeric_value: f64,     // :NOTE: uses https://en.wikipedia.org/wiki/IEEE_754 for math
    #[serde(default)]
    unit: Option<UnitAddress>,
}

impl<'a> QuantityValue {
    pub fn new(numeric_value: f64, unit: Option<UnitAddress>) -> QuantityValue {
        QuantityValue {
            numeric_value,
            unit,
        }
    }

    pub fn get_unit(&'a self) -> Option<UnitAddress> {
        self.unit.to_owned()
    }
}

pub fn add(q1: QuantityValue, q2: QuantityValue) -> QuantityValue {
    if q1.unit != q2.unit {
        panic!("Unimplemented! Need to enable unit conversions in QuantityValue math");
    }
    QuantityValue {
        numeric_value: q1.numeric_value + q2.numeric_value,
        unit: q1.unit,
    }
}

pub fn subtract(q1: QuantityValue, q2: QuantityValue) -> QuantityValue {
    if q1.unit != q2.unit {
        panic!("Unimplemented! Need to enable unit conversions in QuantityValue math");
    }
    QuantityValue {
        numeric_value: q1.numeric_value - q2.numeric_value,
        unit: q1.unit,
    }
}

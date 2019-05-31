use super::type_aliases::UnitAddress;

#[derive(Debug, Default, Clone)]
pub struct Unit {
    id: UnitAddress,
    name: Option<String>,
    symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct QuantityValue {
    numeric_value: f32,  // :TODO: is this going to be wide enough in all cases?
    unit: UnitAddress,
}

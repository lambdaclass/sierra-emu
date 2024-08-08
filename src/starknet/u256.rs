use crate::Value;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct U256 {
    pub lo: u128,
    pub hi: u128,
}

impl U256 {
    pub(crate) fn into_value(self) -> Value {
        Value::Struct(vec![Value::U128(self.lo), Value::U128(self.hi)])
    }
}

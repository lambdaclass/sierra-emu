use super::U256;
use crate::Value;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Secp256k1Point {
    pub x: U256,
    pub y: U256,
}

impl Secp256k1Point {
    #[allow(unused)]
    pub fn into_value(self) -> Value {
        Value::Struct(vec![
            Value::Struct(vec![Value::U128(self.x.lo), Value::U128(self.x.hi)]),
            Value::Struct(vec![Value::U128(self.y.lo), Value::U128(self.y.hi)]),
        ])
    }
}

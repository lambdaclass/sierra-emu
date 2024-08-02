use cairo_lang_sierra::{extensions::core::CoreTypeConcrete, ids::ConcreteTypeId};
use serde::Serialize;
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize)]
pub enum Value {
    Unit,
    Uninitialized(ConcreteTypeId),

    Felt(Felt),
    U128(u128),
}

impl Value {
    pub fn is(&self, type_info: &CoreTypeConcrete) -> bool {
        match type_info {
            CoreTypeConcrete::Felt252(_) => matches!(self, Self::Felt(_)),
            CoreTypeConcrete::RangeCheck(_) | CoreTypeConcrete::SegmentArena(_) => {
                matches!(self, Self::Unit)
            }
            CoreTypeConcrete::GasBuiltin(_) => matches!(self, Self::U128(_)),
            _ => todo!(),
        }
    }

    #[doc(hidden)]
    pub fn parse_felt(value: &str) -> Self {
        Self::Felt(if value.starts_with("0x") {
            Felt::from_hex(value).unwrap()
        } else {
            Felt::from_dec_str(value).unwrap()
        })
    }
}

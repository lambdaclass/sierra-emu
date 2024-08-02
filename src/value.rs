use cairo_lang_sierra::{extensions::core::CoreTypeConcrete, ids::ConcreteTypeId};
use serde::Serialize;
use starknet_types_core::felt::Felt;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Value<'a> {
    Array {
        ty: &'a ConcreteTypeId,
        data: Vec<Self>,
    },
    Felt(Felt),
    FeltDict {
        ty: &'a ConcreteTypeId,
        data: HashMap<Felt, Self>,
    },
    U128(u128),
    U32(u32),
    U8(u8),
    Uninitialized {
        ty: &'a ConcreteTypeId,
    },
    Unit,
}

impl<'a> Value<'a> {
    pub fn is(&self, type_info: &CoreTypeConcrete) -> bool {
        match type_info {
            CoreTypeConcrete::Array(info) => {
                matches!(self, Self::Array { ty, .. } if *ty == &info.ty)
            }
            CoreTypeConcrete::Felt252(_) => matches!(self, Self::Felt(_)),
            CoreTypeConcrete::Felt252Dict(info) => {
                matches!(self, Self::FeltDict { ty, .. } if *ty == &info.ty)
            }
            CoreTypeConcrete::GasBuiltin(_) => matches!(self, Self::U128(_)),
            CoreTypeConcrete::Uint8(_) => matches!(self, Self::U8(_)),

            // Unused builtins (mapped to `Value::Unit`).
            CoreTypeConcrete::RangeCheck(_) | CoreTypeConcrete::SegmentArena(_) => {
                matches!(self, Self::Unit)
            }

            // To do:
            CoreTypeConcrete::Coupon(_) => todo!(),
            CoreTypeConcrete::Bitwise(_) => todo!(),
            CoreTypeConcrete::Box(_) => todo!(),
            CoreTypeConcrete::Circuit(_) => todo!(),
            CoreTypeConcrete::Const(_) => todo!(),
            CoreTypeConcrete::EcOp(_) => todo!(),
            CoreTypeConcrete::EcPoint(_) => todo!(),
            CoreTypeConcrete::EcState(_) => todo!(),
            CoreTypeConcrete::BuiltinCosts(_) => todo!(),
            CoreTypeConcrete::Uint16(_) => todo!(),
            CoreTypeConcrete::Uint32(_) => todo!(),
            CoreTypeConcrete::Uint64(_) => todo!(),
            CoreTypeConcrete::Uint128(_) => todo!(),
            CoreTypeConcrete::Uint128MulGuarantee(_) => todo!(),
            CoreTypeConcrete::Sint8(_) => todo!(),
            CoreTypeConcrete::Sint16(_) => todo!(),
            CoreTypeConcrete::Sint32(_) => todo!(),
            CoreTypeConcrete::Sint64(_) => todo!(),
            CoreTypeConcrete::Sint128(_) => todo!(),
            CoreTypeConcrete::NonZero(_) => todo!(),
            CoreTypeConcrete::Nullable(_) => todo!(),
            CoreTypeConcrete::RangeCheck96(_) => todo!(),
            CoreTypeConcrete::Uninitialized(_) => todo!(),
            CoreTypeConcrete::Enum(_) => todo!(),
            CoreTypeConcrete::Struct(_) => todo!(),
            CoreTypeConcrete::Felt252DictEntry(_) => todo!(),
            CoreTypeConcrete::SquashedFelt252Dict(_) => todo!(),
            CoreTypeConcrete::Pedersen(_) => todo!(),
            CoreTypeConcrete::Poseidon(_) => todo!(),
            CoreTypeConcrete::Span(_) => todo!(),
            CoreTypeConcrete::StarkNet(_) => todo!(),
            CoreTypeConcrete::Snapshot(_) => todo!(),
            CoreTypeConcrete::Bytes31(_) => todo!(),
            CoreTypeConcrete::BoundedInt(_) => todo!(),
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

use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        starknet::StarkNetTypeConcrete,
    },
    ids::ConcreteTypeId,
    program_registry::ProgramRegistry,
};
use num_bigint::BigInt;
use serde::Serialize;
use starknet_types_core::felt::Felt;
use std::{collections::HashMap, fmt::Debug, ops::Range};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Value {
    Array {
        ty: ConcreteTypeId,
        data: Vec<Self>,
    },
    BoundedInt {
        range: Range<BigInt>,
        value: BigInt,
    },
    Enum {
        self_ty: ConcreteTypeId,
        index: usize,
        payload: Box<Self>,
    },
    Felt(Felt),
    FeltDict {
        ty: ConcreteTypeId,
        data: HashMap<Felt, Self>,
    },
    FeltDictEntry {
        ty: ConcreteTypeId,
        data: HashMap<Felt, Self>,
        key: Felt,
    },
    I8(i8),
    Struct(Vec<Self>),
    U128(u128),
    U256(u128, u128),
    U16(u16),
    U32(u32),
    U64(u64),
    U8(u8),
    Uninitialized {
        ty: ConcreteTypeId,
    },
    Unit,
}

impl Value {
    pub fn default_for_type(
        registry: &ProgramRegistry<CoreType, CoreLibfunc>,
        type_id: &ConcreteTypeId,
    ) -> Self {
        match registry.get_type(type_id).unwrap() {
            CoreTypeConcrete::Uint32(_) => Value::U32(0),
            _ => panic!("type {type_id} has no default value implementation"),
        }
    }

    pub fn is(
        &self,
        registry: &ProgramRegistry<CoreType, CoreLibfunc>,
        type_id: &ConcreteTypeId,
    ) -> bool {
        match registry.get_type(type_id).unwrap() {
            CoreTypeConcrete::Array(info) => {
                matches!(self, Self::Array { ty, .. } if *ty == info.ty)
            }
            CoreTypeConcrete::Enum(_) => {
                matches!(self, Self::Enum { self_ty, .. } if self_ty == type_id)
            }
            CoreTypeConcrete::Felt252(_) => matches!(self, Self::Felt(_)),
            CoreTypeConcrete::Felt252Dict(info) => {
                matches!(self, Self::FeltDict { ty, .. } if *ty == info.ty)
            }
            CoreTypeConcrete::GasBuiltin(_) => matches!(self, Self::U128(_)),
            CoreTypeConcrete::NonZero(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::Sint8(_) => matches!(self, Self::I8(_)),
            CoreTypeConcrete::Snapshot(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::Struct(info) => {
                matches!(self, Self::Struct(members)
                    if members.len() == info.members.len()
                        && members
                            .iter()
                            .zip(&info.members)
                            .all(|(value, ty)| value.is(registry, ty))
                )
            }
            CoreTypeConcrete::Uint8(_) => matches!(self, Self::U8(_)),
            CoreTypeConcrete::Uint32(_) => matches!(self, Self::U32(_)),

            // Unused builtins (mapped to `Value::Unit`).
            CoreTypeConcrete::RangeCheck(_) | CoreTypeConcrete::SegmentArena(_) => {
                matches!(self, Self::Unit)
            }

            // To do:
            CoreTypeConcrete::Coupon(_) => todo!(),
            CoreTypeConcrete::Bitwise(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Box(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::Circuit(_) => todo!(),
            CoreTypeConcrete::Const(_) => todo!(),
            CoreTypeConcrete::EcOp(_) => todo!(),
            CoreTypeConcrete::EcPoint(_) => todo!(),
            CoreTypeConcrete::EcState(_) => todo!(),
            CoreTypeConcrete::BuiltinCosts(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Uint16(_) => matches!(self, Self::U16(_)),
            CoreTypeConcrete::Uint64(_) => matches!(self, Self::U64(_)),
            CoreTypeConcrete::Uint128(_) => matches!(self, Self::U128(_)),
            CoreTypeConcrete::Uint128MulGuarantee(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Sint16(_) => todo!(),
            CoreTypeConcrete::Sint32(_) => todo!(),
            CoreTypeConcrete::Sint64(_) => todo!(),
            CoreTypeConcrete::Sint128(_) => todo!(),
            CoreTypeConcrete::Nullable(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::RangeCheck96(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Uninitialized(_) => todo!(),
            CoreTypeConcrete::Felt252DictEntry(_) => todo!(),
            CoreTypeConcrete::SquashedFelt252Dict(_) => todo!(),
            CoreTypeConcrete::Pedersen(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Poseidon(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Span(_) => todo!(),
            CoreTypeConcrete::StarkNet(inner) => match inner {
                StarkNetTypeConcrete::ClassHash(_)
                | StarkNetTypeConcrete::ContractAddress(_)
                | StarkNetTypeConcrete::StorageBaseAddress(_)
                | StarkNetTypeConcrete::StorageAddress(_) => matches!(self, Self::Felt(_)),
                StarkNetTypeConcrete::System(_) => matches!(self, Self::Unit),
                StarkNetTypeConcrete::Secp256Point(_) => todo!(),
                StarkNetTypeConcrete::Sha256StateHandle(_) => todo!(),
            },
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

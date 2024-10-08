use cairo_lang_sierra::{
    extensions::{
        circuit::CircuitTypeConcrete,
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        starknet::StarkNetTypeConcrete,
        ConcreteType,
    },
    ids::ConcreteTypeId,
    program_registry::ProgramRegistry,
};
use num_bigint::{BigInt, BigUint};
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
    Circuit(Vec<BigUint>),
    CircuitModulus(BigUint),
    CircuitOutputs(Vec<BigUint>),
    Enum {
        self_ty: ConcreteTypeId,
        index: usize,
        payload: Box<Self>,
    },
    Felt(Felt),
    Bytes31(Felt),
    FeltDict {
        ty: ConcreteTypeId,
        data: HashMap<Felt, Self>,
    },
    FeltDictEntry {
        ty: ConcreteTypeId,
        data: HashMap<Felt, Self>,
        key: Felt,
    },
    EcPoint {
        x: Felt,
        y: Felt,
    },
    EcState {
        x0: Felt,
        y0: Felt,
        x1: Felt,
        y1: Felt,
    },
    I128(i128),
    I32(i32),
    I8(i8),
    Struct(Vec<Self>),
    U256(u128, u128),
    U128(u128),
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
            CoreTypeConcrete::Uint8(_) => Value::U8(0),
            CoreTypeConcrete::Uint32(_) => Value::U32(0),
            CoreTypeConcrete::Uint64(_) => Value::U64(0),
            CoreTypeConcrete::Uint16(_) => Value::U16(0),
            CoreTypeConcrete::Uint128(_) => Value::U128(0),
            CoreTypeConcrete::Felt252(_) => Value::Felt(0.into()),
            x => panic!("type {:?} has no default value implementation", x.info()),
        }
    }

    pub fn is(
        &self,
        registry: &ProgramRegistry<CoreType, CoreLibfunc>,
        type_id: &ConcreteTypeId,
    ) -> bool {
        let ty = registry.get_type(type_id).unwrap();
        let res = match ty {
            CoreTypeConcrete::Array(info) => {
                matches!(self, Self::Array { ty, .. } if *ty == info.ty)
            }
            CoreTypeConcrete::BoundedInt(info) => {
                matches!(self, Self::BoundedInt { range, .. } if range.start == info.range.lower && range.end == info.range.upper)
            }
            CoreTypeConcrete::Enum(_) => {
                matches!(self, Self::Enum { self_ty, .. } if self_ty == type_id)
            }
            CoreTypeConcrete::Felt252(_) => matches!(self, Self::Felt(_)),
            CoreTypeConcrete::Bytes31(_) => matches!(self, Self::Bytes31(_)),
            CoreTypeConcrete::Felt252Dict(info) => {
                matches!(self, Self::FeltDict { ty, .. } if *ty == info.ty)
            }
            CoreTypeConcrete::GasBuiltin(_) => matches!(self, Self::U128(_)),
            CoreTypeConcrete::NonZero(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::Sint128(_) => matches!(self, Self::I128(_)),
            CoreTypeConcrete::Sint32(_) => matches!(self, Self::I32(_)),
            CoreTypeConcrete::Sint8(_) => matches!(self, Self::I8(_)),
            CoreTypeConcrete::Snapshot(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::StarkNet(
                StarkNetTypeConcrete::ClassHash(_)
                | StarkNetTypeConcrete::ContractAddress(_)
                | StarkNetTypeConcrete::StorageBaseAddress(_)
                | StarkNetTypeConcrete::StorageAddress(_),
            ) => matches!(self, Self::Felt(_)),
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
            CoreTypeConcrete::Uint128(_)
            | CoreTypeConcrete::Circuit(CircuitTypeConcrete::U96Guarantee(_)) => {
                matches!(self, Self::U128(_))
            }

            // Unused builtins (mapped to `Value::Unit`).
            CoreTypeConcrete::RangeCheck(_)
            | CoreTypeConcrete::SegmentArena(_)
            | CoreTypeConcrete::RangeCheck96(_)
            | CoreTypeConcrete::Circuit(
                CircuitTypeConcrete::AddMod(_) | CircuitTypeConcrete::MulMod(_),
            )
            | CoreTypeConcrete::StarkNet(StarkNetTypeConcrete::System(_)) => {
                matches!(self, Self::Unit)
            }

            // To do:
            CoreTypeConcrete::Coupon(_) => todo!(),
            CoreTypeConcrete::Bitwise(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Box(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::Circuit(_) => todo!(),
            CoreTypeConcrete::Const(_) => todo!(),
            CoreTypeConcrete::EcOp(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::EcPoint(_) => matches!(self, Self::EcPoint { .. }),
            CoreTypeConcrete::EcState(_) => matches!(self, Self::EcState { .. }),
            CoreTypeConcrete::BuiltinCosts(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Uint16(_) => matches!(self, Self::U16(_)),
            CoreTypeConcrete::Uint64(_) => matches!(self, Self::U64(_)),
            CoreTypeConcrete::Uint128MulGuarantee(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Sint16(_) => todo!(),
            CoreTypeConcrete::Sint64(_) => todo!(),
            CoreTypeConcrete::Nullable(info) => self.is(registry, &info.ty),
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
        };

        if !res {
            dbg!("value is mismatch", ty.info(), self);
        }

        res
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

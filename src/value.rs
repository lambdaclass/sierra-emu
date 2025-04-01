use cairo_lang_sierra::{
    extensions::{
        circuit::CircuitTypeConcrete,
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        starknet::StarknetTypeConcrete,
        ConcreteType,
    },
    ids::ConcreteTypeId,
    program_registry::ProgramRegistry,
};
use num_bigint::{BigInt, BigUint};
use serde::Serialize;
use starknet_types_core::felt::Felt;
use std::{collections::HashMap, fmt::Debug, ops::Range};

use crate::{debug::type_to_name, gas::BuiltinCosts};

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
    I64(i64),
    I32(i32),
    I16(i16),
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
    BuiltinCosts(BuiltinCosts),
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
            CoreTypeConcrete::Enum(info) => Value::Enum {
                self_ty: type_id.clone(),
                index: 0,
                payload: Box::new(Value::default_for_type(registry, &info.variants[0])),
            },
            CoreTypeConcrete::Struct(info) => Value::Struct(
                info.members
                    .iter()
                    .map(|member| Value::default_for_type(registry, member))
                    .collect(),
            ),
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
            CoreTypeConcrete::GasBuiltin(_) => matches!(self, Self::U64(_)),
            CoreTypeConcrete::NonZero(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::Sint128(_) => matches!(self, Self::I128(_)),
            CoreTypeConcrete::Sint32(_) => matches!(self, Self::I32(_)),
            CoreTypeConcrete::Sint8(_) => matches!(self, Self::I8(_)),
            CoreTypeConcrete::Snapshot(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::Starknet(
                StarknetTypeConcrete::ClassHash(_)
                | StarknetTypeConcrete::ContractAddress(_)
                | StarknetTypeConcrete::StorageBaseAddress(_)
                | StarknetTypeConcrete::StorageAddress(_),
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
            CoreTypeConcrete::Uint128(_) => {
                matches!(self, Self::U128(_))
            }

            // Unused builtins (mapped to `Value::Unit`).
            CoreTypeConcrete::RangeCheck(_)
            | CoreTypeConcrete::SegmentArena(_)
            | CoreTypeConcrete::RangeCheck96(_)
            | CoreTypeConcrete::Starknet(StarknetTypeConcrete::System(_)) => {
                matches!(self, Self::Unit)
            }

            // To do:
            CoreTypeConcrete::Coupon(_) => todo!(),
            CoreTypeConcrete::Bitwise(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Box(info) => self.is(registry, &info.ty),

            // Circuit related types
            CoreTypeConcrete::Circuit(selector) => match selector {
                CircuitTypeConcrete::Circuit(_) => matches!(self, Self::Circuit(_)),
                CircuitTypeConcrete::CircuitData(_) => matches!(self, Self::Circuit(_)),
                CircuitTypeConcrete::CircuitOutputs(_) => matches!(self, Self::CircuitOutputs(_)),
                CircuitTypeConcrete::CircuitInput(_) => matches!(self, Self::Unit),
                CircuitTypeConcrete::CircuitInputAccumulator(_) => matches!(self, Self::Circuit(_)),
                CircuitTypeConcrete::CircuitModulus(_) => matches!(self, Self::CircuitModulus(_)),
                CircuitTypeConcrete::U96Guarantee(_) => matches!(self, Self::U128(_)),
                CircuitTypeConcrete::CircuitDescriptor(_)
                | CircuitTypeConcrete::CircuitFailureGuarantee(_)
                | CircuitTypeConcrete::AddMod(_)
                | CircuitTypeConcrete::MulMod(_)
                | CircuitTypeConcrete::AddModGate(_)
                | CircuitTypeConcrete::CircuitPartialOutputs(_)
                | CircuitTypeConcrete::InverseGate(_)
                | CircuitTypeConcrete::MulModGate(_)
                | CircuitTypeConcrete::SubModGate(_)
                | CircuitTypeConcrete::U96LimbsLessThanGuarantee(_) => {
                    matches!(self, Self::Unit)
                }
            },
            CoreTypeConcrete::Const(_) => todo!(),
            CoreTypeConcrete::EcOp(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::EcPoint(_) => matches!(self, Self::EcPoint { .. }),
            CoreTypeConcrete::EcState(_) => matches!(self, Self::EcState { .. }),
            CoreTypeConcrete::BuiltinCosts(_) => matches!(self, Self::BuiltinCosts(_)),
            CoreTypeConcrete::Uint16(_) => matches!(self, Self::U16(_)),
            CoreTypeConcrete::Uint64(_) => matches!(self, Self::U64(_)),
            CoreTypeConcrete::Uint128MulGuarantee(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Sint16(_) => matches!(self, Self::I16(_)),
            CoreTypeConcrete::Sint64(_) => matches!(self, Self::I64(_)),
            CoreTypeConcrete::Nullable(info) => self.is(registry, &info.ty),
            CoreTypeConcrete::Uninitialized(_) => matches!(self, Self::Uninitialized { .. }),
            CoreTypeConcrete::Felt252DictEntry(info) => {
                matches!(self, Self::FeltDictEntry { ty, .. } if *ty == info.ty)
            }
            CoreTypeConcrete::SquashedFelt252Dict(info) => {
                matches!(self, Self::FeltDict { ty, .. } if *ty == info.ty)
            }
            CoreTypeConcrete::Pedersen(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Poseidon(_) => matches!(self, Self::Unit),
            CoreTypeConcrete::Span(_) => todo!(),
            CoreTypeConcrete::Starknet(inner) => match inner {
                StarknetTypeConcrete::ClassHash(_)
                | StarknetTypeConcrete::ContractAddress(_)
                | StarknetTypeConcrete::StorageBaseAddress(_)
                | StarknetTypeConcrete::StorageAddress(_) => matches!(self, Self::Felt(_)),
                StarknetTypeConcrete::System(_) => matches!(self, Self::Unit),
                StarknetTypeConcrete::Secp256Point(_) => matches!(self, Self::Struct(_)),
                StarknetTypeConcrete::Sha256StateHandle(_) => matches!(self, Self::Struct { .. }),
            },
            CoreTypeConcrete::IntRange(_) => todo!(),
            CoreTypeConcrete::Blake(_) => todo!(),
            CoreTypeConcrete::QM31(_) => todo!(),
        };

        if !res {
            dbg!(
                "value is mismatch",
                ty.info(),
                self,
                type_to_name(type_id, registry)
            );
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

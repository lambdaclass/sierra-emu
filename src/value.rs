use cairo_lang_sierra::{
    extensions::{
        circuit::CircuitTypeConcrete,
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        starknet::StarkNetTypeConcrete,
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
    CircuitOutputs(HashMap<u64, BigUint>),
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
    U384(BigUint),
    U256(u128, u128),
    U128(u128),
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
            CoreTypeConcrete::BoundedInt(info) => {
                matches!(self, Self::BoundedInt { range, .. } if range.start == info.range.lower && range.end == info.range.upper)
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
            _ => todo!("implement `Value::is` for type {type_id}"),
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

use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        const_type::{ConstAsImmediateConcreteLibfunc, ConstConcreteLibfunc},
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
    },
    ids::ConcreteTypeId,
    program::GenericArg,
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &ConstConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        ConstConcreteLibfunc::AsBox(_) => todo!(),
        ConstConcreteLibfunc::AsImmediate(info) => eval_as_immediate(registry, info, args),
    }
}

pub fn eval_as_immediate(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &ConstAsImmediateConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [] = args.try_into().unwrap();

    fn inner(
        registry: &ProgramRegistry<CoreType, CoreLibfunc>,
        type_id: &ConcreteTypeId,
        inner_data: &[GenericArg],
    ) -> Value {
        match registry.get_type(type_id).unwrap() {
            CoreTypeConcrete::BoundedInt(info) => match inner_data {
                [GenericArg::Type(type_id)] => match registry.get_type(type_id).unwrap() {
                    CoreTypeConcrete::Const(info) => {
                        inner(registry, &info.inner_ty, &info.inner_data)
                    }
                    _ => unreachable!(),
                },
                [GenericArg::Value(value)] => {
                    assert!(value >= &info.range.lower && value < &info.range.upper);
                    Value::BoundedInt {
                        range: info.range.lower.clone()..info.range.upper.clone(),
                        value: value.clone(),
                    }
                }
                _ => unreachable!(),
            },
            CoreTypeConcrete::Felt252(_) => match inner_data {
                [GenericArg::Value(value)] => Value::Felt(value.into()),
                _ => unreachable!(),
            },
            CoreTypeConcrete::NonZero(info) => inner(registry, &info.ty, inner_data),
            CoreTypeConcrete::Sint128(_) => match inner_data {
                [GenericArg::Value(value)] => Value::I128(value.try_into().unwrap()),
                _ => unreachable!(),
            },
            CoreTypeConcrete::Sint32(_) => match inner_data {
                [GenericArg::Value(value)] => Value::I32(value.try_into().unwrap()),
                _ => unreachable!(),
            },
            CoreTypeConcrete::Sint8(_) => match inner_data {
                [GenericArg::Value(value)] => Value::I8(value.try_into().unwrap()),
                _ => unreachable!(),
            },
            CoreTypeConcrete::Uint32(_) => match inner_data {
                [GenericArg::Value(value)] => Value::U32(value.try_into().unwrap()),
                _ => unreachable!(),
            },
            CoreTypeConcrete::Uint8(_) => match inner_data {
                [GenericArg::Value(value)] => Value::U8(value.try_into().unwrap()),
                _ => unreachable!(),
            },
            CoreTypeConcrete::Uint128(_) => match inner_data {
                [GenericArg::Value(value)] => Value::U128(value.try_into().unwrap()),
                _ => unreachable!(),
            },
            CoreTypeConcrete::Struct(_) => {
                let mut fields = Vec::new();

                for field in inner_data {
                    match field {
                        GenericArg::Type(const_field_ty) => {
                            let field_type = registry.get_type(const_field_ty).unwrap();

                            match &field_type {
                                CoreTypeConcrete::Const(const_ty) => {
                                    let field_value =
                                        inner(registry, &const_ty.inner_ty, &const_ty.inner_data);
                                    fields.push(field_value);
                                }
                                _ => unreachable!(),
                            };
                        }
                        _ => unreachable!(),
                    }
                }

                Value::Struct(fields)
            }
            CoreTypeConcrete::Array(_) => todo!("1"),
            CoreTypeConcrete::Coupon(_) => todo!("2"),
            CoreTypeConcrete::Bitwise(_) => todo!("3"),
            CoreTypeConcrete::Box(_) => todo!("4"),
            CoreTypeConcrete::Circuit(_) => todo!("5"),
            CoreTypeConcrete::Const(_) => todo!("6"),
            CoreTypeConcrete::EcOp(_) => todo!("7"),
            CoreTypeConcrete::EcPoint(_) => todo!("8"),
            CoreTypeConcrete::EcState(_) => todo!("9"),
            CoreTypeConcrete::GasBuiltin(_) => todo!("10"),
            CoreTypeConcrete::BuiltinCosts(_) => todo!("11"),
            CoreTypeConcrete::Uint16(_) => todo!("12"),
            CoreTypeConcrete::Uint64(_) => match inner_data {
                [GenericArg::Value(value)] => Value::U64(value.try_into().unwrap()),
                _ => unreachable!(),
            },
            CoreTypeConcrete::Uint128MulGuarantee(_) => todo!("14"),
            CoreTypeConcrete::Sint16(_) => todo!("15"),
            CoreTypeConcrete::Sint64(_) => todo!("17"),
            CoreTypeConcrete::Nullable(_) => todo!("19"),
            CoreTypeConcrete::RangeCheck(_) => todo!("20"),
            CoreTypeConcrete::RangeCheck96(_) => todo!("21"),
            CoreTypeConcrete::Uninitialized(_) => todo!("22"),
            CoreTypeConcrete::Enum(_) => todo!("23"),
            CoreTypeConcrete::Felt252Dict(_) => todo!("24"),
            CoreTypeConcrete::Felt252DictEntry(_) => todo!("25"),
            CoreTypeConcrete::SquashedFelt252Dict(_) => todo!("26"),
            CoreTypeConcrete::Pedersen(_) => todo!("27"),
            CoreTypeConcrete::Poseidon(_) => todo!("28"),
            CoreTypeConcrete::Span(_) => todo!("29"),
            CoreTypeConcrete::StarkNet(_) => todo!("30"),
            CoreTypeConcrete::SegmentArena(_) => todo!("31"),
            CoreTypeConcrete::Snapshot(_) => todo!("32"),
            CoreTypeConcrete::Bytes31(_) => todo!("33"),
        }
    }

    let const_ty = match registry.get_type(&info.const_type).unwrap() {
        CoreTypeConcrete::Const(x) => x,
        _ => unreachable!(),
    };
    EvalAction::NormalBranch(
        0,
        smallvec![inner(registry, &const_ty.inner_ty, &const_ty.inner_data)],
    )
}

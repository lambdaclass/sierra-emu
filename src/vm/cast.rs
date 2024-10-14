use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        casts::{CastConcreteLibfunc, DowncastConcreteLibfunc},
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::SignatureOnlyConcreteLibfunc,
        ConcreteType,
    },
    program_registry::ProgramRegistry,
};
use num_bigint::BigInt;
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &CastConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        CastConcreteLibfunc::Downcast(info) => eval_downcast(registry, info, args),
        CastConcreteLibfunc::Upcast(info) => eval_upcast(registry, info, args),
    }
}

pub fn eval_downcast(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &DowncastConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, value]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    let value = match value {
        Value::BoundedInt { value, .. } => value,
        Value::U128(value) => BigInt::from(value),
        Value::U64(value) => BigInt::from(value),
        Value::U32(value) => BigInt::from(value),
        Value::U16(value) => BigInt::from(value),
        Value::U8(value) => BigInt::from(value),
        _ => todo!(),
    };

    let range = info.to_range.lower.clone()..info.to_range.upper.clone();
    if range.contains(&value) {
        EvalAction::NormalBranch(
            0,
            smallvec![
                range_check,
                match registry.get_type(&info.to_ty).unwrap() {
                    CoreTypeConcrete::Sint8(_) => Value::I8(value.try_into().unwrap()),
                    CoreTypeConcrete::Sint128(_) => Value::I128(value.try_into().unwrap()),
                    CoreTypeConcrete::Uint8(_) => Value::U8(value.try_into().unwrap()),
                    CoreTypeConcrete::Uint16(_) => Value::U16(value.try_into().unwrap()),
                    CoreTypeConcrete::Uint32(_) => Value::U32(value.try_into().unwrap()),
                    CoreTypeConcrete::Uint64(_) => Value::U64(value.try_into().unwrap()),
                    CoreTypeConcrete::Uint128(_) => Value::U128(value.try_into().unwrap()),
                    CoreTypeConcrete::BoundedInt(_) => Value::BoundedInt { range, value },
                    x => todo!("{:?}", x.info()),
                }
            ],
        )
    } else {
        EvalAction::NormalBranch(1, smallvec![range_check])
    }
}

pub fn eval_upcast(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();

    let value = match value {
        Value::BoundedInt { value, .. } => value,
        Value::U128(value) => BigInt::from(value),
        Value::U64(value) => BigInt::from(value),
        Value::U32(value) => BigInt::from(value),
        Value::U16(value) => BigInt::from(value),
        Value::U8(value) => BigInt::from(value),
        _ => todo!(),
    };

    EvalAction::NormalBranch(
        0,
        smallvec![match registry
            .get_type(&info.signature.branch_signatures[0].vars[0].ty)
            .unwrap()
        {
            CoreTypeConcrete::Sint8(_) => Value::I8(value.try_into().unwrap()),
            CoreTypeConcrete::Sint32(_) => Value::I32(value.try_into().unwrap()),
            CoreTypeConcrete::Sint128(_) => Value::I128(value.try_into().unwrap()),
            CoreTypeConcrete::Uint8(_) => Value::U8(value.try_into().unwrap()),
            CoreTypeConcrete::Uint16(_) => Value::U16(value.try_into().unwrap()),
            CoreTypeConcrete::Uint32(_) => Value::U32(value.try_into().unwrap()),
            CoreTypeConcrete::Uint64(_) => Value::U64(value.try_into().unwrap()),
            CoreTypeConcrete::Uint128(_) => Value::U128(value.try_into().unwrap()),
            CoreTypeConcrete::Felt252(_) => Value::Felt(value.try_into().unwrap()),
            CoreTypeConcrete::Array(_) => todo!("Array"),
            CoreTypeConcrete::Coupon(_) => todo!("Coupon"),
            CoreTypeConcrete::Bitwise(_) => todo!("Bitwise"),
            CoreTypeConcrete::Box(_) => todo!("Box"),
            CoreTypeConcrete::Circuit(_) => todo!("Circuit"),
            CoreTypeConcrete::Const(_) => todo!("Const"),
            CoreTypeConcrete::EcOp(_) => todo!("EcOp"),
            CoreTypeConcrete::EcPoint(_) => todo!("EcPoint"),
            CoreTypeConcrete::EcState(_) => todo!("EcState"),
            CoreTypeConcrete::GasBuiltin(_) => todo!("GasBuiltin"),
            CoreTypeConcrete::BuiltinCosts(_) => todo!("BuiltinCosts"),
            CoreTypeConcrete::Uint128MulGuarantee(_) => todo!("Uint128MulGuarantee"),
            CoreTypeConcrete::Sint16(_) => todo!("Sint16"),
            CoreTypeConcrete::Sint64(_) => todo!("Sint64"),
            CoreTypeConcrete::NonZero(_) => todo!("NonZero"),
            CoreTypeConcrete::Nullable(_) => todo!("Nullable"),
            CoreTypeConcrete::RangeCheck(_) => todo!("RangeCheck"),
            CoreTypeConcrete::RangeCheck96(_) => todo!("RangeCheck96"),
            CoreTypeConcrete::Uninitialized(_) => todo!("Uninitialized"),
            CoreTypeConcrete::Enum(_) => todo!("Enum"),
            CoreTypeConcrete::Struct(_) => todo!("Struct"),
            CoreTypeConcrete::Felt252Dict(_) => todo!("Felt252Dict"),
            CoreTypeConcrete::Felt252DictEntry(_) => todo!("Felt252DictEntry"),
            CoreTypeConcrete::SquashedFelt252Dict(_) => todo!("SquashedFelt252Dict"),
            CoreTypeConcrete::Pedersen(_) => todo!("Pedersen"),
            CoreTypeConcrete::Poseidon(_) => todo!("Poseidon"),
            CoreTypeConcrete::Span(_) => todo!("Span"),
            CoreTypeConcrete::StarkNet(_) => todo!("StarkNet"),
            CoreTypeConcrete::SegmentArena(_) => todo!("SegmentArena"),
            CoreTypeConcrete::Snapshot(_) => todo!("Snapshot"),
            CoreTypeConcrete::Bytes31(_) => todo!("Bytes31"),
            CoreTypeConcrete::BoundedInt(_) => todo!("BoundedInt"),
        }],
    )
}

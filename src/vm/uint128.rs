use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        int::{
            unsigned128::{Uint128Concrete, Uint128Traits},
            IntConstConcreteLibfunc, IntOperationConcreteLibfunc, IntOperator,
        },
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;
use starknet_crypto::Felt;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Uint128Concrete,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        Uint128Concrete::Const(info) => eval_const(registry, info, args),
        Uint128Concrete::Operation(info) => eval_operation(registry, info, args),
        Uint128Concrete::SquareRoot(_) => todo!(),
        Uint128Concrete::Equal(info) => eval_equal(registry, info, args),
        Uint128Concrete::ToFelt252(info) => eval_to_felt(registry, info, args),
        Uint128Concrete::FromFelt252(info) => eval_from_felt(registry, info, args),
        Uint128Concrete::IsZero(info) => eval_is_zero(registry, info, args),
        Uint128Concrete::Divmod(_) => todo!(),
        Uint128Concrete::Bitwise(info) => eval_bitwise(registry, info, args),
        Uint128Concrete::GuaranteeMul(_) => todo!(),
        Uint128Concrete::MulGuaranteeVerify(_) => todo!(),
        Uint128Concrete::ByteReverse(_) => todo!(),
    }
}

pub fn eval_operation(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &IntOperationConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::U128(lhs), Value::U128(rhs)]: [Value; 3] =
        args.try_into().unwrap()
    else {
        panic!()
    };

    let (result, has_overflow) = match info.operator {
        IntOperator::OverflowingAdd => lhs.overflowing_add(rhs),
        IntOperator::OverflowingSub => lhs.overflowing_sub(rhs),
    };

    EvalAction::NormalBranch(
        has_overflow as usize,
        smallvec![range_check, Value::U128(result)],
    )
}

pub fn eval_equal(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::U128(lhs), Value::U128(rhs)]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    EvalAction::NormalBranch((lhs == rhs) as usize, smallvec![])
}

pub fn eval_is_zero(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [vm_value @ Value::U128(value)]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    if value == 0 {
        EvalAction::NormalBranch(0, smallvec![])
    } else {
        EvalAction::NormalBranch(1, smallvec![vm_value])
    }
}

pub fn eval_bitwise(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [bitwise @ Value::Unit, Value::U128(lhs), Value::U128(rhs)]: [Value; 3] =
        args.try_into().unwrap()
    else {
        panic!()
    };

    let and = lhs & rhs;
    let or = lhs | rhs;
    let xor = lhs ^ rhs;

    EvalAction::NormalBranch(
        0,
        smallvec![bitwise, Value::U128(and), Value::U128(or), Value::U128(xor)],
    )
}

pub fn eval_const(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &IntConstConcreteLibfunc<Uint128Traits>,
    _args: Vec<Value>,
) -> EvalAction {
    EvalAction::NormalBranch(0, smallvec![Value::U128(info.c)])
}

pub fn eval_to_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::U128(value)]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    EvalAction::NormalBranch(0, smallvec![Value::Felt(value.into())])
}

pub fn eval_from_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::Felt(value)]: [Value; 2] = args.try_into().unwrap()
    else {
        panic!()
    };

    let max = Felt::from(u128::MAX);

    if value <= max {
        let value: u128 = value.to_biguint().try_into().unwrap();
        EvalAction::NormalBranch(0, smallvec![range_check, Value::U128(value)])
    } else {
        let overflow: u128 = (value - max).to_biguint().try_into().unwrap();
        let value: u128 = max.to_biguint().try_into().unwrap();
        EvalAction::NormalBranch(
            1,
            smallvec![range_check, Value::U128(value), Value::U128(overflow)],
        )
    }
}

use std::u128;

use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        int::unsigned256::Uint256Concrete,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use num_bigint::BigUint;
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Uint256Concrete,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        Uint256Concrete::IsZero(info) => eval_is_zero(registry, info, args),
        Uint256Concrete::Divmod(info) => eval_divmod(registry, info, args),
        Uint256Concrete::SquareRoot(_) => todo!(),
        Uint256Concrete::InvModN(_) => todo!(),
    }
}

pub fn eval_is_zero(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Struct(fields)]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    let [Value::U128(lo), Value::U128(hi)]: [Value; 2] = fields.clone().try_into().unwrap() else {
        panic!()
    };

    if lo == 0 && hi == 0 {
        EvalAction::NormalBranch(0, smallvec![])
    } else {
        EvalAction::NormalBranch(1, smallvec![Value::Struct(fields)])
    }
}

#[inline]
pub fn u256_to_biguint(lo: u128, hi: u128) -> BigUint {
    BigUint::from(lo) + (BigUint::from(hi) << 128)
}

#[inline]
pub fn u256_to_value(value: BigUint) -> Value {
    let hi: u128 = (&value >> 128u32).try_into().unwrap();
    let lo: u128 = (value & BigUint::from(u128::MAX)).try_into().unwrap();
    Value::Struct(vec![Value::U128(lo), Value::U128(hi)])
}

pub fn eval_divmod(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::Struct(lhs), Value::Struct(rhs)]: [Value; 3] =
        args.try_into().unwrap()
    else {
        panic!()
    };

    let [Value::U128(lhs_lo), Value::U128(lhs_hi)]: [Value; 2] = lhs.try_into().unwrap() else {
        panic!()
    };

    let lhs = u256_to_biguint(lhs_lo, lhs_hi);

    let [Value::U128(rhs_lo), Value::U128(rhs_hi)]: [Value; 2] = rhs.try_into().unwrap() else {
        panic!()
    };

    let rhs = u256_to_biguint(rhs_lo, rhs_hi);

    let div = &lhs / &rhs;
    let modulo = lhs % rhs;

    EvalAction::NormalBranch(
        0,
        smallvec![
            range_check,
            u256_to_value(div),
            u256_to_value(modulo),
            Value::Unit
        ],
    )
}
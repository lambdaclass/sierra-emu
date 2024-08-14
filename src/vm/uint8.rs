use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        int::{
            unsigned::{Uint8Concrete, Uint8Traits},
            IntConstConcreteLibfunc, IntOperationConcreteLibfunc, IntOperator,
        },
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Uint8Concrete,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        Uint8Concrete::Const(info) => eval_const(registry, info, args),
        Uint8Concrete::Operation(info) => eval_operation(registry, info, args),
        Uint8Concrete::SquareRoot(_) => todo!(),
        Uint8Concrete::Equal(info) => eval_equal(registry, info, args),
        Uint8Concrete::ToFelt252(_) => todo!(),
        Uint8Concrete::FromFelt252(_) => todo!(),
        Uint8Concrete::IsZero(info) => eval_is_zero(registry, info, args),
        Uint8Concrete::Divmod(_) => todo!(),
        Uint8Concrete::WideMul(info) => eval_widemul(registry, info, args),
        Uint8Concrete::Bitwise(_) => todo!(),
    }
}

pub fn eval_operation(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &IntOperationConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::U8(lhs), Value::U8(rhs)]: [Value; 3] =
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
        smallvec![range_check, Value::U8(result)],
    )
}

pub fn eval_equal(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::U8(lhs), Value::U8(rhs)]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    EvalAction::NormalBranch((lhs != rhs) as usize, smallvec![])
}

pub fn eval_is_zero(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [vm_value @ Value::U8(value)]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    if value == 0 {
        EvalAction::NormalBranch(0, smallvec![])
    } else {
        EvalAction::NormalBranch(1, smallvec![vm_value])
    }
}

pub fn eval_const(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &IntConstConcreteLibfunc<Uint8Traits>,
    _args: Vec<Value>,
) -> EvalAction {
    EvalAction::NormalBranch(0, smallvec![Value::U8(info.c)])
}

pub fn eval_widemul(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::U8(lhs), Value::U8(rhs)]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    let result = (lhs as u16) * (rhs as u16);

    EvalAction::NormalBranch(0, smallvec![Value::U16(result)])
}

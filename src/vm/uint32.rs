use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        int::{unsigned::Uint32Concrete, IntOperationConcreteLibfunc, IntOperator},
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Uint32Concrete,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        Uint32Concrete::Const(_) => todo!(),
        Uint32Concrete::Operation(info) => eval_operation(registry, info, args),
        Uint32Concrete::SquareRoot(_) => todo!(),
        Uint32Concrete::Equal(_) => todo!(),
        Uint32Concrete::ToFelt252(info) => eval_to_felt252(registry, info, args),
        Uint32Concrete::FromFelt252(_) => todo!(),
        Uint32Concrete::IsZero(_) => todo!(),
        Uint32Concrete::Divmod(_) => todo!(),
        Uint32Concrete::WideMul(_) => todo!(),
        Uint32Concrete::Bitwise(_) => todo!(),
    }
}

pub fn eval_operation(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &IntOperationConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::U32(lhs), Value::U32(rhs)]: [Value; 3] =
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
        smallvec![range_check, Value::U32(result)],
    )
}

pub fn eval_to_felt252(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::U32(value)]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    EvalAction::NormalBranch(0, smallvec![Value::Felt(value.into())])
}

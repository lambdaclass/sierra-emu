use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        int::{unsigned::Uint32Concrete, IntOperationConcreteLibfunc, IntOperator},
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &'a Uint32Concrete,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    match selector {
        Uint32Concrete::Const(_) => todo!(),
        Uint32Concrete::Operation(info) => eval_operation(registry, info, args),
        Uint32Concrete::SquareRoot(_) => todo!(),
        Uint32Concrete::Equal(_) => todo!(),
        Uint32Concrete::ToFelt252(_) => todo!(),
        Uint32Concrete::FromFelt252(_) => todo!(),
        Uint32Concrete::IsZero(_) => todo!(),
        Uint32Concrete::Divmod(_) => todo!(),
        Uint32Concrete::WideMul(_) => todo!(),
        Uint32Concrete::Bitwise(_) => todo!(),
    }
}

pub fn eval_operation<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    info: &'a IntOperationConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [range_check @ Value::Unit, Value::U32(lhs), Value::U32(rhs)]: [Value<'a>; 3] =
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

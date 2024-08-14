use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        int::{
            unsigned::{Uint8Concrete, Uint8Traits},
            IntConstConcreteLibfunc,
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
        Uint8Concrete::Operation(_) => todo!(),
        Uint8Concrete::SquareRoot(_) => todo!(),
        Uint8Concrete::Equal(info) => eval_equal(registry, info, args),
        Uint8Concrete::ToFelt252(_) => todo!(),
        Uint8Concrete::FromFelt252(_) => todo!(),
        Uint8Concrete::IsZero(_) => todo!(),
        Uint8Concrete::Divmod(_) => todo!(),
        Uint8Concrete::WideMul(_) => todo!(),
        Uint8Concrete::Bitwise(_) => todo!(),
    }
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

pub fn eval_const(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &IntConstConcreteLibfunc<Uint8Traits>,
    _args: Vec<Value>,
) -> EvalAction {
    EvalAction::NormalBranch(0, smallvec![Value::U8(info.c)])
}

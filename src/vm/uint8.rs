use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        int::unsigned::Uint8Concrete,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &'a Uint8Concrete,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    match selector {
        Uint8Concrete::Const(_) => todo!(),
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

pub fn eval_equal<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureOnlyConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [Value::U8(lhs), Value::U8(rhs)]: [Value<'a>; 2] = args.try_into().unwrap() else {
        panic!()
    };

    EvalAction::NormalBranch((lhs != rhs) as usize, smallvec![])
}

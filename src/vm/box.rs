use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        boxing::BoxConcreteLibfunc,
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureAndTypeConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &'a BoxConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    match selector {
        BoxConcreteLibfunc::Into(_) => todo!(),
        BoxConcreteLibfunc::Unbox(info) => eval_unbox(registry, info, args),
        BoxConcreteLibfunc::ForwardSnapshot(_) => todo!(),
    }
}

pub fn eval_unbox<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [value] = args.try_into().unwrap();

    EvalAction::NormalBranch(0, smallvec![value])
}

use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureOnlyConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [] = args.try_into().unwrap();

    EvalAction::NormalBranch(0, smallvec![])
}

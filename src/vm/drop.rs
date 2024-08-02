use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;

pub fn eval<'a>(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: &[Value<'a>],
) -> EvalAction<'a> {
    assert_eq!(args.len(), 1);
    EvalAction::NormalBranch(0, vec![])
}

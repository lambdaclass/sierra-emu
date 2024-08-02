use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        ap_tracking::ApTrackingConcreteLibfunc,
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &'a ApTrackingConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    match selector {
        ApTrackingConcreteLibfunc::Revoke(_) => todo!(),
        ApTrackingConcreteLibfunc::Enable(info) => eval_disable(registry, info, args),
        ApTrackingConcreteLibfunc::Disable(info) => eval_disable(registry, info, args),
    }
}

pub fn eval_enable<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureOnlyConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [] = args.try_into().unwrap();

    EvalAction::NormalBranch(0, smallvec![])
}

pub fn eval_disable<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureOnlyConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [] = args.try_into().unwrap();

    EvalAction::NormalBranch(0, smallvec![])
}

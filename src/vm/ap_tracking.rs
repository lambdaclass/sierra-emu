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
    args: &[Value<'a>],
) -> EvalAction<'a> {
    match selector {
        ApTrackingConcreteLibfunc::Revoke(_) => todo!(),
        ApTrackingConcreteLibfunc::Enable(_) => todo!(),
        ApTrackingConcreteLibfunc::Disable(info) => eval_disable(registry, info, args),
    }
}

pub fn eval_disable<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureOnlyConcreteLibfunc,
    args: &[Value<'a>],
) -> EvalAction<'a> {
    assert!(args.is_empty());
    EvalAction::NormalBranch(0, smallvec![])
}

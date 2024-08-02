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

pub fn eval<'a>(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &ApTrackingConcreteLibfunc,
    args: &[Value],
) -> EvalAction<'a> {
    match selector {
        ApTrackingConcreteLibfunc::Revoke(_) => todo!(),
        ApTrackingConcreteLibfunc::Enable(_) => todo!(),
        ApTrackingConcreteLibfunc::Disable(info) => eval_disable(registry, info, args),
    }
}

pub fn eval_disable<'a>(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: &[Value],
) -> EvalAction<'a> {
    assert!(args.is_empty());
    EvalAction::NormalBranch(0, vec![])
}

use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        ap_tracking::ApTrackingConcreteLibfunc,
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &ApTrackingConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    match selector {
        ApTrackingConcreteLibfunc::Revoke(_) => todo!(),
        ApTrackingConcreteLibfunc::Enable(_) => todo!(),
        ApTrackingConcreteLibfunc::Disable(info) => eval_disable(registry, info, args), // <-- Implement this.
    }
}

pub fn eval_disable(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert!(args.is_empty());
    (Some(0), vec![])
}

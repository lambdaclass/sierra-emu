use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        ap_tracking::ApTrackingConcreteLibfunc,
        core::{CoreLibfunc, CoreType},
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
        ApTrackingConcreteLibfunc::Disable(_) => todo!(), // <-- Implement this.
    }
}

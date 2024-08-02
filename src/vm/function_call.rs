use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        function_call::SignatureAndFunctionConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;

pub fn eval<'a>(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _selector: &SignatureAndFunctionConcreteLibfunc,
    _args: &[Value],
) -> EvalAction<'a> {
    todo!()
}

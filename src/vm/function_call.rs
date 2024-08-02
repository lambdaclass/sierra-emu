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
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _selector: &'a SignatureAndFunctionConcreteLibfunc,
    _args: &[Value<'a>],
) -> EvalAction<'a> {
    todo!()
}

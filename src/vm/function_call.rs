use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        function_call::SignatureAndFunctionConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;

pub fn eval(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _selector: &SignatureAndFunctionConcreteLibfunc,
    _args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    todo!()
}

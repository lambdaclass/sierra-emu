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
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    info: &'a SignatureAndFunctionConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    assert_eq!(args.len(), info.function.params.len());
    assert!(args
        .iter()
        .zip(&info.function.params)
        .all(|(value, param)| value.is(registry, &param.ty)));

    EvalAction::FunctionCall(&info.function.id, args.into_iter().collect())
}

use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;

pub fn eval(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert_eq!(args.len(), 1);
    (Some(0), vec![args[0].clone(), args[0].clone()])
}

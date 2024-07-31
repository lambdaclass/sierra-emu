use crate::value::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureAndTypeConcreteLibfunc,
        mem::MemConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &MemConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    match selector {
        MemConcreteLibfunc::StoreTemp(info) => eval_store_temp(registry, info, args),
        MemConcreteLibfunc::StoreLocal(_) => todo!(),
        MemConcreteLibfunc::FinalizeLocals(_) => todo!(),
        MemConcreteLibfunc::AllocLocal(_) => todo!(),
        MemConcreteLibfunc::Rename(_) => todo!(),
    }
}

pub fn eval_store_temp(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert_eq!(args.len(), 1);
    (Some(0), args.to_vec())
}

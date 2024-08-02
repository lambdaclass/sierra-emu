use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        lib_func::{SignatureAndTypeConcreteLibfunc, SignatureOnlyConcreteLibfunc},
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
        MemConcreteLibfunc::FinalizeLocals(info) => eval_finalize_locals(registry, info, args),
        MemConcreteLibfunc::AllocLocal(info) => eval_alloc_local(registry, info, args),
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

pub fn eval_finalize_locals(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert!(args.is_empty());
    (Some(0), vec![])
}

pub fn eval_alloc_local(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndTypeConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert!(args.is_empty());
    (Some(0), vec![Value::Uninitialized(info.ty.clone())])
}
use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        lib_func::{SignatureAndTypeConcreteLibfunc, SignatureOnlyConcreteLibfunc},
        mem::MemConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &'a MemConcreteLibfunc,
    args: &[Value<'a>],
) -> EvalAction<'a> {
    match selector {
        MemConcreteLibfunc::StoreTemp(info) => eval_store_temp(registry, info, args),
        MemConcreteLibfunc::StoreLocal(info) => eval_store_local(registry, info, args),
        MemConcreteLibfunc::FinalizeLocals(info) => eval_finalize_locals(registry, info, args),
        MemConcreteLibfunc::AllocLocal(info) => eval_alloc_local(registry, info, args),
        MemConcreteLibfunc::Rename(_) => todo!(),
    }
}

pub fn eval_store_temp<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureAndTypeConcreteLibfunc,
    args: &[Value<'a>],
) -> EvalAction<'a> {
    assert_eq!(args.len(), 1);
    EvalAction::NormalBranch(0, args.iter().cloned().collect())
}

pub fn eval_store_local<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureAndTypeConcreteLibfunc,
    args: &[Value<'a>],
) -> EvalAction<'a> {
    assert_eq!(args.len(), 2);

    let type_id = match &args[0] {
        Value::Uninitialized { ty } => ty,
        _ => unreachable!(),
    };
    assert!(
        args[1].is(registry, type_id),
        "{:?} is not a {:?}",
        args[1],
        type_id
    );

    EvalAction::NormalBranch(0, smallvec![args[1].clone()])
}

pub fn eval_finalize_locals<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureOnlyConcreteLibfunc,
    args: &[Value<'a>],
) -> EvalAction<'a> {
    assert!(args.is_empty());
    EvalAction::NormalBranch(0, smallvec![])
}

pub fn eval_alloc_local<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    info: &'a SignatureAndTypeConcreteLibfunc,
    args: &[Value<'a>],
) -> EvalAction<'a> {
    assert!(args.is_empty());
    EvalAction::NormalBranch(0, smallvec![Value::Uninitialized { ty: &info.ty }])
}

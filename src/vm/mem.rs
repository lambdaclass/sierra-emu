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
    args: Vec<Value<'a>>,
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
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [value] = args.try_into().unwrap();

    EvalAction::NormalBranch(0, smallvec![value])
}

pub fn eval_store_local<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureAndTypeConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [Value::Uninitialized { ty }, value]: [Value<'a>; 2] = args.try_into().unwrap() else {
        panic!()
    };
    assert!(value.is(registry, ty));

    EvalAction::NormalBranch(0, smallvec![value])
}

pub fn eval_finalize_locals<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureOnlyConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [] = args.try_into().unwrap();

    EvalAction::NormalBranch(0, smallvec![])
}

pub fn eval_alloc_local<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    info: &'a SignatureAndTypeConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [] = args.try_into().unwrap();

    EvalAction::NormalBranch(0, smallvec![Value::Uninitialized { ty: &info.ty }])
}

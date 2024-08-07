use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::SignatureOnlyConcreteLibfunc,
        structure::{StructConcreteLibfunc, StructConcreteType},
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &StructConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        StructConcreteLibfunc::Construct(info) => eval_construct(registry, info, args),
        StructConcreteLibfunc::Deconstruct(_) => todo!(),
        StructConcreteLibfunc::SnapshotDeconstruct(_) => todo!(),
    }
}

pub fn eval_construct(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let CoreTypeConcrete::Struct(StructConcreteType { members, .. }) = registry
        .get_type(&info.signature.branch_signatures[0].vars[0].ty)
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(args.len(), members.len());
    assert!(args
        .iter()
        .zip(members)
        .all(|(value, ty)| value.is(registry, ty)));

    EvalAction::NormalBranch(0, smallvec![Value::Struct(args)])
}

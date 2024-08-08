use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        enm::{EnumConcreteLibfunc, EnumConcreteType, EnumInitConcreteLibfunc},
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &EnumConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        EnumConcreteLibfunc::Init(info) => eval_init(registry, info, args),
        EnumConcreteLibfunc::FromBoundedInt(_) => todo!(),
        EnumConcreteLibfunc::Match(info) => eval_match(registry, info, args),
        EnumConcreteLibfunc::SnapshotMatch(_) => todo!(),
    }
}

pub fn eval_init(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &EnumInitConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();

    let self_ty = &info.signature.branch_signatures[0].vars[0].ty;
    let CoreTypeConcrete::Enum(EnumConcreteType { variants, .. }) =
        registry.get_type(self_ty).unwrap()
    else {
        panic!()
    };
    assert_eq!(info.n_variants, variants.len());
    assert!(info.index < info.n_variants);
    assert!(value.is(registry, &variants[info.index]));

    EvalAction::NormalBranch(
        0,
        smallvec![Value::Enum {
            self_ty: self_ty.clone(),
            index: info.index,
            payload: Box::new(value),
        }],
    )
}

pub fn eval_match(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Enum {
        self_ty,
        index,
        payload,
    }]: [Value; 1] = args.try_into().unwrap()
    else {
        panic!()
    };
    assert_eq!(self_ty, info.signature.param_signatures[0].ty);
    assert!(payload.is(
        registry,
        &info.signature.branch_signatures[index].vars[0].ty
    ));

    EvalAction::NormalBranch(index, smallvec![*payload])
}

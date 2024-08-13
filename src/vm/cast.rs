use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        casts::{CastConcreteLibfunc, DowncastConcreteLibfunc},
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &CastConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        CastConcreteLibfunc::Downcast(info) => eval_downcast(registry, info, args),
        CastConcreteLibfunc::Upcast(info) => eval_upcast(registry, info, args),
    }
}

pub fn eval_downcast(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &DowncastConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, value]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    let value = match value {
        Value::BoundedInt { value, .. } => value,
        _ => todo!(),
    };

    let range = info.to_range.lower.clone()..info.to_range.upper.clone();
    if range.contains(&value) {
        EvalAction::NormalBranch(
            0,
            smallvec![
                range_check,
                match registry.get_type(&info.to_ty).unwrap() {
                    CoreTypeConcrete::Sint8(_) => Value::I8(value.try_into().unwrap()),
                    _ => todo!(),
                }
            ],
        )
    } else {
        EvalAction::NormalBranch(1, smallvec![range_check])
    }
}

pub fn eval_upcast(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    dbg!(info
        .signature
        .param_signatures
        .iter()
        .map(|x| x.ty.to_string())
        .collect::<Vec<_>>());
    dbg!(info
        .signature
        .branch_signatures
        .iter()
        .map(|x| x.vars.iter().map(|x| x.ty.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>());
    dbg!(&args);

    let [value] = args.try_into().unwrap();

    let value = match value {
        Value::BoundedInt { value, .. } => value,
        _ => todo!(),
    };

    EvalAction::NormalBranch(
        0,
        smallvec![match registry
            .get_type(&info.signature.branch_signatures[0].vars[0].ty)
            .unwrap()
        {
            CoreTypeConcrete::Sint8(_) => Value::I8(value.try_into().unwrap()),
            _ => todo!(),
        }],
    )
}

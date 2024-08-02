use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        array::ArrayConcreteLibfunc,
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::{SignatureAndTypeConcreteLibfunc, SignatureOnlyConcreteLibfunc},
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &'a ArrayConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    match selector {
        ArrayConcreteLibfunc::New(info) => eval_new(registry, info, args),
        ArrayConcreteLibfunc::SpanFromTuple(_) => todo!(),
        ArrayConcreteLibfunc::TupleFromSpan(_) => todo!(),
        ArrayConcreteLibfunc::Append(info) => eval_append(registry, info, args),
        ArrayConcreteLibfunc::PopFront(_) => todo!(),
        ArrayConcreteLibfunc::PopFrontConsume(_) => todo!(),
        ArrayConcreteLibfunc::Get(_) => todo!(),
        ArrayConcreteLibfunc::Slice(_) => todo!(),
        ArrayConcreteLibfunc::Len(_) => todo!(),
        ArrayConcreteLibfunc::SnapshotPopFront(_) => todo!(),
        ArrayConcreteLibfunc::SnapshotPopBack(_) => todo!(),
        ArrayConcreteLibfunc::SnapshotMultiPopFront(_) => todo!(),
        ArrayConcreteLibfunc::SnapshotMultiPopBack(_) => todo!(),
    }
}

pub fn eval_new<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [] = args.try_into().unwrap();

    let type_info = registry
        .get_type(&info.signature.branch_signatures[0].vars[0].ty)
        .unwrap();
    let ty = match type_info {
        CoreTypeConcrete::Array(info) => &info.ty,
        _ => unreachable!(),
    };

    EvalAction::NormalBranch(
        0,
        smallvec![Value::Array {
            ty,
            data: Vec::new(),
        }],
    )
}

pub fn eval_append<'a>(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [Value::Array { ty, mut data }, item]: [Value<'a>; 2] = args.try_into().unwrap() else {
        panic!()
    };

    assert_eq!(&info.signature.param_signatures[1].ty, ty);
    assert!(item.is(registry, ty));
    data.push(item.clone());

    EvalAction::NormalBranch(0, smallvec![Value::Array { ty, data }])
}

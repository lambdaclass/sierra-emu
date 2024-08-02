use cairo_lang_sierra::{
    extensions::{
        array::ArrayConcreteLibfunc,
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::{SignatureAndTypeConcreteLibfunc, SignatureOnlyConcreteLibfunc},
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &ArrayConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
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

pub fn eval_new(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert!(args.is_empty());

    let type_info = registry
        .get_type(&info.signature.branch_signatures[0].vars[0].ty)
        .unwrap();
    let ty = match type_info {
        CoreTypeConcrete::Array(info) => info.ty.clone(),
        _ => unreachable!(),
    };

    (
        Some(0),
        vec![Value::Array {
            ty,
            data: Vec::new(),
        }],
    )
}

pub fn eval_append(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndTypeConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert_eq!(args.len(), 2);

    let (ty, mut data) = match &args[0] {
        Value::Array { ty, data: values } => (ty.clone(), values.clone()),
        _ => todo!(),
    };

    assert_eq!(info.signature.param_signatures[1].ty, ty);
    assert!(args[1].is(registry.get_type(&ty).unwrap()));
    data.push(args[1].clone());

    (Some(0), vec![Value::Array { ty, data }])
}

use cairo_lang_sierra::{
    extensions::{
        array::ArrayConcreteLibfunc,
        core::{CoreLibfunc, CoreType},
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
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert!(args.is_empty());

    (
        Some(0),
        vec![Value::Array {
            ty: info.signature.branch_signatures[0].vars[0].ty.clone(),
            values: Vec::new(),
        }],
    )
}

pub fn eval_append(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndTypeConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert_eq!(args.len(), 2);

    let (ty, mut values) = match &args[0] {
        Value::Array { ty, values } => (ty.clone(), values.clone()),
        _ => todo!(),
    };

    assert_eq!(info.signature.param_signatures[0].ty, ty);
    values.push(args[1].clone());

    (Some(0), vec![Value::Array { ty, values }])
}

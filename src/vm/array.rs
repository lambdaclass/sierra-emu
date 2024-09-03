use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        array::ArrayConcreteLibfunc,
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::{SignatureAndTypeConcreteLibfunc, SignatureOnlyConcreteLibfunc},
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &ArrayConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        ArrayConcreteLibfunc::New(info) => eval_new(registry, info, args),
        ArrayConcreteLibfunc::SpanFromTuple(_) => todo!(),
        ArrayConcreteLibfunc::TupleFromSpan(_) => todo!(),
        ArrayConcreteLibfunc::Append(info) => eval_append(registry, info, args),
        ArrayConcreteLibfunc::PopFront(info) => eval_pop_front(registry, info, args),
        ArrayConcreteLibfunc::PopFrontConsume(_) => todo!(),
        ArrayConcreteLibfunc::Get(info) => eval_get(registry, info, args),
        ArrayConcreteLibfunc::Slice(info) => eval_slice(registry, info, args),
        ArrayConcreteLibfunc::Len(info) => eval_len(registry, info, args),
        ArrayConcreteLibfunc::SnapshotPopFront(info) => {
            eval_snapshot_pop_front(registry, info, args)
        }
        ArrayConcreteLibfunc::SnapshotPopBack(info) => eval_snapshot_pop_back(registry, info, args),
        ArrayConcreteLibfunc::SnapshotMultiPopFront(_) => todo!(),
        ArrayConcreteLibfunc::SnapshotMultiPopBack(_) => todo!(),
    }
}

pub fn eval_new(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
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
            ty: ty.clone(),
            data: Vec::new(),
        }],
    )
}

pub fn eval_append(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Array { ty, mut data }, item]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    assert_eq!(info.signature.param_signatures[1].ty, ty);
    assert!(item.is(registry, &ty));
    data.push(item.clone());

    EvalAction::NormalBranch(0, smallvec![Value::Array { ty, data }])
}

pub fn eval_get(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::Array { data, .. }, Value::U32(index)]: [Value; 3] =
        args.try_into().unwrap()
    else {
        panic!()
    };

    match data.get(index as usize).cloned() {
        Some(value) => EvalAction::NormalBranch(0, smallvec![range_check, value]),
        None => EvalAction::NormalBranch(1, smallvec![range_check]),
    }
}

pub fn eval_slice(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::Array { data, ty }, Value::U32(start), Value::U32(len)]: [Value; 4] =
        args.try_into().unwrap()
    else {
        panic!()
    };

    match data.get(start as usize..(start + len) as usize) {
        Some(value) => EvalAction::NormalBranch(
            0,
            smallvec![
                range_check,
                Value::Array {
                    data: value.to_vec(),
                    ty
                }
            ],
        ),
        None => EvalAction::NormalBranch(1, smallvec![range_check]),
    }
}

pub fn eval_len(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Array { data, .. }]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    let array_len = data.len().try_into().unwrap();
    EvalAction::NormalBranch(0, smallvec![Value::U32(array_len)])
}

pub fn eval_pop_front(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Array { mut data, ty }]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    if !data.is_empty() {
        let new_data = data.split_off(1);
        let value = data[0].clone();
        EvalAction::NormalBranch(0, smallvec![Value::Array { data: new_data, ty }, value])
    } else {
        EvalAction::NormalBranch(1, smallvec![Value::Array { data, ty }])
    }
}

pub fn eval_snapshot_pop_front(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Array { mut data, ty }]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    if !data.is_empty() {
        let new_data = data.split_off(1);
        let value = data[0].clone();
        assert!(value.is(registry, &info.ty));
        EvalAction::NormalBranch(0, smallvec![Value::Array { data: new_data, ty }, value])
    } else {
        EvalAction::NormalBranch(1, smallvec![Value::Array { data, ty }])
    }
}

pub fn eval_snapshot_pop_back(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Array { mut data, ty }]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    if !data.is_empty() {
        let new_data = data.split_off(data.len() - 1);
        let value = new_data[0].clone();
        assert!(value.is(registry, &info.ty));
        EvalAction::NormalBranch(0, smallvec![Value::Array { data, ty }, value])
    } else {
        EvalAction::NormalBranch(1, smallvec![Value::Array { data, ty }])
    }
}

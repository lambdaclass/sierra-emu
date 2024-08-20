use std::ops::Range;

use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        bytes31::Bytes31ConcreteLibfunc,
        consts::SignatureAndConstConcreteLibfunc,
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::SignatureOnlyConcreteLibfunc,
        ConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Bytes31ConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        Bytes31ConcreteLibfunc::Const(info) => eval_const(registry, info, args),
        Bytes31ConcreteLibfunc::ToFelt252(info) => eval_to_felt252(registry, info, args),
        Bytes31ConcreteLibfunc::TryFromFelt252(info) => eval_from_felt(registry, info, args),
    }
}

pub fn eval_const(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndConstConcreteLibfunc,
    _args: Vec<Value>,
) -> EvalAction {
    let out_ty = registry
        .get_type(&info.branch_signatures()[0].vars[0].ty)
        .unwrap();

    if let CoreTypeConcrete::BoundedInt(bounded_info) = out_ty {
        let range = bounded_info.range.clone();
        let value = Value::BoundedInt {
            range: Range {
                start: range.lower,
                end: range.upper,
            },
            value: info.c.clone(),
        };
        EvalAction::NormalBranch(0, smallvec![value])
    } else {
        panic!()
    }
}

pub fn eval_from_felt(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::Felt(value)]: [Value; 2] = args.try_into().unwrap()
    else {
        panic!()
    };

    let out_ty = registry
        .get_type(&info.branch_signatures()[0].vars[1].ty)
        .unwrap();

    if let CoreTypeConcrete::BoundedInt(info) = out_ty {
        let range = info.range.clone();

        if range.is_full_felt252_range()
            || (value < range.upper.clone().into() && value >= range.lower.clone().into())
        {
            let value = Value::BoundedInt {
                range: Range {
                    start: range.lower,
                    end: range.upper,
                },
                value: value.to_bigint(),
            };
            EvalAction::NormalBranch(0, smallvec![range_check, value])
        } else {
            EvalAction::NormalBranch(1, smallvec![range_check])
        }
    } else {
        panic!()
    }
}

pub fn eval_to_felt252(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::BoundedInt { range: _, value }]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    EvalAction::NormalBranch(0, smallvec![Value::Felt(value.into())])
}

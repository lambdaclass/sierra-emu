use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        bounded_int::{
            BoundedIntConcreteLibfunc, BoundedIntConstrainConcreteLibfunc,
            BoundedIntDivRemConcreteLibfunc,
        },
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::SignatureOnlyConcreteLibfunc,
        ConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &BoundedIntConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        BoundedIntConcreteLibfunc::Add(_) => todo!(),
        BoundedIntConcreteLibfunc::Sub(_) => todo!(),
        BoundedIntConcreteLibfunc::Mul(info) => eval_mul(registry, info, args),
        BoundedIntConcreteLibfunc::DivRem(info) => eval_div_rem(registry, info, args),
        BoundedIntConcreteLibfunc::Constrain(info) => eval_constrain(registry, info, args),
        BoundedIntConcreteLibfunc::IsZero(info) => eval_is_zero(registry, info, args),
        BoundedIntConcreteLibfunc::WrapNonZero(_) => todo!(),
    }
}

pub fn eval_mul(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::BoundedInt { value: lhs, .. }, Value::BoundedInt { value: rhs, .. }]: [Value; 2] =
        args.try_into().unwrap()
    else {
        panic!()
    };

    let range = match registry
        .get_type(&info.signature.branch_signatures[0].vars[0].ty)
        .unwrap()
    {
        CoreTypeConcrete::BoundedInt(info) => info.range.lower.clone()..info.range.upper.clone(),
        CoreTypeConcrete::NonZero(info) => match registry.get_type(&info.ty).unwrap() {
            CoreTypeConcrete::BoundedInt(info) => {
                info.range.lower.clone()..info.range.upper.clone()
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    EvalAction::NormalBranch(
        0,
        smallvec![Value::BoundedInt {
            range,
            value: lhs * rhs,
        }],
    )
}

pub fn eval_div_rem(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &BoundedIntDivRemConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::BoundedInt {
        range: lhs_range,
        value: lhs,
    }, Value::BoundedInt {
        range: rhs_range,
        value: rhs,
    }]: [Value; 3] = args.try_into().unwrap()
    else {
        panic!()
    };
    assert_eq!(lhs_range.start, info.lhs.lower);
    assert_eq!(lhs_range.end, info.lhs.upper);
    assert_eq!(rhs_range.start, info.rhs.lower);
    assert_eq!(rhs_range.end, info.rhs.upper);

    let quo = &lhs / &rhs;
    let rem = lhs % rhs;

    let quo_range = match registry
        .get_type(&info.branch_signatures()[0].vars[1].ty)
        .unwrap()
    {
        CoreTypeConcrete::BoundedInt(info) => info.range.lower.clone()..info.range.upper.clone(),
        _ => unreachable!(),
    };
    let rem_range = match registry
        .get_type(&info.branch_signatures()[0].vars[2].ty)
        .unwrap()
    {
        CoreTypeConcrete::BoundedInt(info) => info.range.lower.clone()..info.range.upper.clone(),
        _ => unreachable!(),
    };
    assert!(quo_range.contains(&quo));
    assert!(rem_range.contains(&rem));

    EvalAction::NormalBranch(
        0,
        smallvec![
            range_check,
            Value::BoundedInt {
                range: quo_range,
                value: quo,
            },
            Value::BoundedInt {
                range: rem_range,
                value: rem,
            },
        ],
    )
}

pub fn eval_constrain(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &BoundedIntConstrainConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, value]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    let value = match value {
        Value::I8(value) => value.into(),
        _ => todo!(),
    };

    if value < info.boundary {
        let range = match registry
            .get_type(&info.branch_signatures()[0].vars[1].ty)
            .unwrap()
        {
            CoreTypeConcrete::BoundedInt(info) => {
                info.range.lower.clone()..info.range.upper.clone()
            }
            CoreTypeConcrete::NonZero(info) => match registry.get_type(&info.ty).unwrap() {
                CoreTypeConcrete::BoundedInt(info) => {
                    info.range.lower.clone()..info.range.upper.clone()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        EvalAction::NormalBranch(
            0,
            smallvec![range_check, Value::BoundedInt { range, value }],
        )
    } else {
        let range = match registry
            .get_type(&info.branch_signatures()[1].vars[1].ty)
            .unwrap()
        {
            CoreTypeConcrete::BoundedInt(info) => {
                info.range.lower.clone()..info.range.upper.clone()
            }
            CoreTypeConcrete::NonZero(info) => match registry.get_type(&info.ty).unwrap() {
                CoreTypeConcrete::BoundedInt(info) => {
                    info.range.lower.clone()..info.range.upper.clone()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        EvalAction::NormalBranch(
            1,
            smallvec![range_check, Value::BoundedInt { range, value }],
        )
    }
}

pub fn eval_is_zero(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();
    let is_zero = match value {
        Value::I8(value) => value == 0,
        _ => todo!(),
    };

    if is_zero {
        EvalAction::NormalBranch(0, smallvec![])
    } else {
        EvalAction::NormalBranch(1, smallvec![value])
    }
}

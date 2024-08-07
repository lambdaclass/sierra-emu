use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        const_type::{ConstAsImmediateConcreteLibfunc, ConstConcreteLibfunc},
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
    },
    program::GenericArg,
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &ConstConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        ConstConcreteLibfunc::AsBox(_) => todo!(),
        ConstConcreteLibfunc::AsImmediate(info) => eval_as_immediate(registry, info, args),
    }
}

pub fn eval_as_immediate(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &ConstAsImmediateConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [] = args.try_into().unwrap();

    let const_ty = match registry.get_type(&info.const_type).unwrap() {
        CoreTypeConcrete::Const(x) => x,
        _ => unreachable!(),
    };

    let value = match registry.get_type(&const_ty.inner_ty).unwrap() {
        CoreTypeConcrete::Felt252(_) => match const_ty.inner_data.as_slice() {
            [GenericArg::Value(value)] => Value::Felt(value.into()),
            _ => unreachable!(),
        },
        CoreTypeConcrete::Uint32(_) => match const_ty.inner_data.as_slice() {
            [GenericArg::Value(value)] => Value::U32(value.try_into().unwrap()),
            _ => unreachable!(),
        },
        CoreTypeConcrete::Uint8(_) => match const_ty.inner_data.as_slice() {
            [GenericArg::Value(value)] => Value::U8(value.try_into().unwrap()),
            _ => unreachable!(),
        },
        _ => todo!("{:?}", &const_ty.inner_ty),
    };

    EvalAction::NormalBranch(0, smallvec![value])
}

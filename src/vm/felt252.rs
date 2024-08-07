use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        felt252::{Felt252BinaryOperationConcrete, Felt252BinaryOperator, Felt252Concrete},
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Felt252Concrete,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        Felt252Concrete::BinaryOperation(info) => eval_squash(registry, info, args),
        Felt252Concrete::Const(_) => todo!(),
        Felt252Concrete::IsZero(_) => todo!(),
    }
}

pub fn eval_squash(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &Felt252BinaryOperationConcrete,
    args: Vec<Value>,
) -> EvalAction {
    let res = match info {
        Felt252BinaryOperationConcrete::WithVar(info) => {
            let [Value::Felt(lhs), Value::Felt(rhs)]: [Value; 2] = args.try_into().unwrap() else {
                panic!()
            };

            match info.operator {
                Felt252BinaryOperator::Add => lhs + rhs,
                Felt252BinaryOperator::Sub => lhs - rhs,
                Felt252BinaryOperator::Mul => lhs * rhs,
                Felt252BinaryOperator::Div => lhs.field_div(&rhs.try_into().unwrap()),
            }
        }
        Felt252BinaryOperationConcrete::WithConst(_info) => todo!(),
    };

    EvalAction::NormalBranch(0, smallvec![Value::Felt(res)])
}

use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        gas::GasConcreteLibfunc,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &GasConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        GasConcreteLibfunc::WithdrawGas(info) => eval_withdraw_gas(registry, info, args),
        GasConcreteLibfunc::RedepositGas(_) => todo!(),
        GasConcreteLibfunc::GetAvailableGas(_) => todo!(),
        GasConcreteLibfunc::BuiltinWithdrawGas(_) => todo!(),
        GasConcreteLibfunc::GetBuiltinCosts(_) => todo!(),
    }
}

pub fn eval_withdraw_gas(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::U128(gas)]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    // TODO: Implement properly.
    EvalAction::NormalBranch(0, smallvec![range_check, Value::U128(gas)])
}

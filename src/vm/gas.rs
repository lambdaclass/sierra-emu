use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        gas::GasConcreteLibfunc,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &'a GasConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    match selector {
        GasConcreteLibfunc::WithdrawGas(info) => eval_withdraw_gas(registry, info, args),
        GasConcreteLibfunc::RedepositGas(_) => todo!(),
        GasConcreteLibfunc::GetAvailableGas(_) => todo!(),
        GasConcreteLibfunc::BuiltinWithdrawGas(_) => todo!(),
        GasConcreteLibfunc::GetBuiltinCosts(_) => todo!(),
    }
}

pub fn eval_withdraw_gas<'a>(
    _registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &'a SignatureOnlyConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [range_check @ Value::Unit, Value::U128(gas)]: [Value<'a>; 2] = args.try_into().unwrap()
    else {
        panic!()
    };

    // TODO: Implement properly.
    EvalAction::NormalBranch(0, smallvec![range_check, Value::U128(gas)])
}

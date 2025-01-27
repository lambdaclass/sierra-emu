use super::EvalAction;
use crate::{
    gas::{BuiltinCosts, GasMetadata},
    Value,
};
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        gas::GasConcreteLibfunc,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program::StatementIdx,
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &GasConcreteLibfunc,
    args: Vec<Value>,
    gas: &GasMetadata,
    statement_idx: StatementIdx,
    builtin_costs: BuiltinCosts
) -> EvalAction {
    match selector {
        GasConcreteLibfunc::WithdrawGas(info) => {
            eval_withdraw_gas(registry, info, args, gas, statement_idx)
        }
        GasConcreteLibfunc::RedepositGas(info) => {
            eval_redeposit_gas(registry, info, args, gas, statement_idx)
        }
        GasConcreteLibfunc::GetAvailableGas(_) => todo!(),
        GasConcreteLibfunc::BuiltinWithdrawGas(info) => {
            eval_builtin_withdraw_gas(registry, info, args, gas, statement_idx)
        }
        GasConcreteLibfunc::GetBuiltinCosts(info) => eval_get_builtin_costs(registry, info, args, builtin_costs),
    }
}

pub fn eval_builtin_withdraw_gas(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    gas_meta: &GasMetadata,
    statement_idx: StatementIdx,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::U64(gas), _builtin_costs @ Value::Unit]: [Value; 3] =
        args.try_into().unwrap()
    else {
        panic!()
    };

    let gas_cost = gas_meta.get_gas_cost_for_statement(statement_idx);

    if let Some(gas_cost) = gas_cost {
        let new_gas = gas.saturating_sub(gas_cost);
        if gas >= gas_cost {
            EvalAction::NormalBranch(0, smallvec![range_check, Value::U64(new_gas)])
        } else {
            EvalAction::NormalBranch(1, smallvec![range_check, Value::U64(gas)])
        }
    } else {
        EvalAction::NormalBranch(1, smallvec![range_check, Value::U64(gas)])
    }
}

pub fn eval_withdraw_gas(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    gas_meta: &GasMetadata,
    statement_idx: StatementIdx,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::U64(gas)]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    let gas_cost = gas_meta.get_gas_cost_for_statement(statement_idx);

    if let Some(gas_cost) = gas_cost {
        let new_gas = gas.saturating_sub(gas_cost);
        if gas >= gas_cost {
            EvalAction::NormalBranch(0, smallvec![range_check, Value::U64(new_gas)])
        } else {
            EvalAction::NormalBranch(1, smallvec![range_check, Value::U64(gas)])
        }
    } else {
        EvalAction::NormalBranch(1, smallvec![range_check, Value::U64(gas)])
    }
}

pub fn eval_redeposit_gas(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    gas_meta: &GasMetadata,
    statement_idx: StatementIdx,
) -> EvalAction {
    let [Value::U64(gas)]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    match gas_meta.get_gas_cost_for_statement(statement_idx) {
        Some(c) => {
            let new_gas = gas.saturating_add(c);

            EvalAction::NormalBranch(0, smallvec![Value::U64(new_gas)])
        }
        None => EvalAction::NormalBranch(0, smallvec![Value::U64(gas)]),
    }
}

pub fn eval_get_builtin_costs(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    _args: Vec<Value>,
    builtin_costs: BuiltinCosts,
) -> EvalAction {
    EvalAction::NormalBranch(0, smallvec![Value::BuiltinCosts(builtin_costs)])
}

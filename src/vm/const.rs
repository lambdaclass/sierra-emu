use crate::value::Value;
use cairo_lang_sierra::{
    extensions::{
        const_type::{ConstAsImmediateConcreteLibfunc, ConstConcreteLibfunc},
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
    },
    program::GenericArg,
    program_registry::ProgramRegistry,
};
use num_bigint::{BigUint, Sign};
use starknet_types_core::felt::Felt;
use std::str::FromStr;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &ConstConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    match selector {
        ConstConcreteLibfunc::AsBox(_) => todo!(),
        ConstConcreteLibfunc::AsImmediate(info) => eval_as_immediate(registry, info, args),
    }
}

pub fn eval_as_immediate(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &ConstAsImmediateConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    let const_ty = match registry.get_type(&info.const_type).unwrap() {
        CoreTypeConcrete::Const(x) => x,
        _ => unreachable!(),
    };

    let value = match registry.get_type(&const_ty.inner_ty).unwrap() {
        CoreTypeConcrete::Felt252(_) => match const_ty.inner_data.as_slice() {
            [GenericArg::Value(value)] => {
                let (sign, mut value) = value.clone().into_parts();
                if sign == Sign::Minus {
                    let prime = BigUint::from_str(
                        "0x0800000000000011000000000000000000000000000000000000000000000001",
                    )
                    .unwrap();

                    value = prime - value;
                }

                Value::Felt(Felt::from_bytes_le_slice(&value.to_bytes_le()))
            }
            _ => unreachable!(),
        },
        _ => todo!(),
    };

    (Some(0), vec![value])
}

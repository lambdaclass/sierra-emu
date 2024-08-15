use std::collections::HashMap;

use super::EvalAction;
use crate::{vm::circuit, Value};
use cairo_lang_sierra::{
    extensions::{
        circuit::{CircuitConcreteLibfunc, CircuitTypeConcrete},
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::{SignatureAndTypeConcreteLibfunc, SignatureOnlyConcreteLibfunc},
    }, ids::ConcreteTypeId, program_registry::ProgramRegistry
};
use num_bigint::{BigInt, BigUint};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &CircuitConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        CircuitConcreteLibfunc::AddInput(info) => eval_add_input(registry, info, args),
        CircuitConcreteLibfunc::Eval(info) => eval_eval(registry, info, args),
        CircuitConcreteLibfunc::GetDescriptor(info) => eval_get_descriptor(registry, info, args),
        CircuitConcreteLibfunc::InitCircuitData(info) => {
            eval_init_circuit_data(registry, info, args)
        }
        CircuitConcreteLibfunc::GetOutput(_) => todo!(),
        CircuitConcreteLibfunc::TryIntoCircuitModulus(info) => {
            eval_try_into_circuit_modulus(registry, info, args)
        }
        CircuitConcreteLibfunc::FailureGuaranteeVerify(_) => todo!(),
        CircuitConcreteLibfunc::IntoU96Guarantee(info) => {
            eval_into_u96_guarantee(registry, info, args)
        }
        CircuitConcreteLibfunc::U96GuaranteeVerify(_) => todo!(),
        CircuitConcreteLibfunc::U96LimbsLessThanGuaranteeVerify(_) => todo!(),
        CircuitConcreteLibfunc::U96SingleLimbLessThanGuaranteeVerify(_) => todo!(),
    }
}

pub fn eval_add_input(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Circuit(mut values), Value::Struct(members)]: [Value; 2] = args.try_into().unwrap()
    else {
        panic!()
    };
    assert_ne!(values.len(), values.capacity());

    let [Value::U128(l0), Value::U128(l1), Value::U128(l2), Value::U128(l3)]: [Value; 4] =
        members.try_into().unwrap()
    else {
        panic!()
    };

    let l0 = l0.to_le_bytes();
    let l1 = l1.to_le_bytes();
    let l2 = l2.to_le_bytes();
    let l3 = l3.to_le_bytes();
    values.push(BigUint::from_bytes_le(&[
        l0[0], l0[1], l0[2], l0[3], l0[4], l0[5], l0[6], l0[7], //
        l1[0], l1[1], l1[2], l1[3], l1[4], l1[5], l1[6], l1[7], //
        l2[0], l2[1], l2[2], l2[3], l2[4], l2[5], l2[6], l2[7], //
        l3[0], l3[1], l3[2], l3[3], l3[4], l3[5], l3[6], l3[7], //
    ]));

    EvalAction::NormalBranch(
        (values.len() != values.capacity()) as usize,
        smallvec![Value::Circuit(values)],
    )
}

pub fn eval_eval(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    _args: Vec<Value>,
) -> EvalAction {
    dbg!(
        "PARAMS: {}",
        _info.signature
            .param_signatures
            .iter()
            .map(|x| &x.ty)
            .collect::<Vec<_>>()
    );
    dbg!(
        "BRANCH: {}",
        _info.signature
            .branch_signatures
            .iter()
            .map(|x| x.vars.iter().map(|x| &x.ty).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
    
    let [add_mod @ Value::Unit, mul_mod @ Value::Unit, descripctor @ Value::Unit, Value::Circuit(inputs), Value::CircuitModulus(module), Value::BoundedInt {
        range: r0,
        value: v0,
    }, Value::BoundedInt {
        range: r1,
        value: v1,
    }]: [Value; 7] = _args.try_into().unwrap()
    else {
        panic!()
    };
    let circ_info = match _registry.get_type(&_info.ty).unwrap() {
        CoreTypeConcrete::Circuit(CircuitTypeConcrete::Circuit(info)) => &info.circuit_info,
        _ => todo!()
    };
    let mut outputs: HashMap<u64, BigUint> = inputs.into_iter().enumerate().map(|(i, input)| {
        let gate = circ_info.mul_offsets[i].output as u64;
        (gate, input)
    }).collect::<HashMap<u64, BigUint>>();
    
    dbg!("ADD: {}", &circ_info.add_offsets);
    dbg!("MUL: {}", &circ_info.mul_offsets);

    let (mut add_gates, mut next_add_gate) = {
        let mut iter = circ_info.add_offsets.iter().peekable();
        let next = iter.next().unwrap();

        (iter, next)
    };
    let (mut mul_gates, mut next_mul_gate) = {
        let mut iter = circ_info.mul_offsets.iter().skip(circ_info.n_inputs).peekable();
        let next = iter.next().unwrap();

        (iter, next)
    };

    for _ in circ_info.n_inputs..circ_info.values.len() {
        let sum_add_gate_args = next_add_gate.lhs + next_add_gate.rhs + next_add_gate.output;
        let sum_mul_gate_args = next_mul_gate.lhs + next_mul_gate.rhs + next_mul_gate.output;
        dbg!(&outputs);
        if mul_gates.peek().is_none() || sum_add_gate_args < sum_mul_gate_args {
            let lhs = outputs.get(&(next_add_gate.lhs as u64)).unwrap();
            let rhs = outputs.get(&(next_add_gate.rhs as u64)).unwrap();
            
            outputs.insert(next_add_gate.output as u64, (lhs + rhs) % &module);
            
            next_add_gate = add_gates.next().unwrap();
        } else if add_gates.peek().is_none() || sum_add_gate_args > sum_mul_gate_args {
            dbg!(next_mul_gate.rhs);
            let rhs = outputs.get(&(next_mul_gate.rhs as u64)).unwrap();
            
            if next_mul_gate.output == 0 {
                outputs.insert(next_mul_gate.lhs as u64, (BigUint::from(1_u64) / rhs) % &module);
            } else {
                let lhs = outputs.get(&(next_mul_gate.lhs as u64)).unwrap();
                outputs.insert(next_mul_gate.output as u64, (lhs * rhs) % &module);
            }
            next_mul_gate = mul_gates.next().unwrap();
        } else {
            break;
        }
    }

    // Params:
    //   - AddMod
    //   - MulMod
    //   - CircuitDescriptor<Circuit<(AddModGate<CircuitInput<0>, CircuitInput<1>>)>>
    //   - CircuitData<Circuit<(AddModGate<CircuitInput<0>, CircuitInput<1>>)>>
    //   - CircuitModulus
    //   - ???
    //   - ???
    //
    // Branches:
    //   [0]:
    //     - AddMod
    //     - MulMod
    //     - CircuitOutputs<Circuit<(AddModGate<CircuitInput<0>, CircuitInput<1>>)>>
    //   [1]:
    //     - AddMod
    //     - MulMod
    //     - CircuitOutputs<Circuit<(AddModGate<CircuitInput<0>, CircuitInput<1>>)>>
    //     - CircuitFailureGuarantee

    todo!()
}

pub fn eval_get_descriptor(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    _args: Vec<Value>,
) -> EvalAction {
    EvalAction::NormalBranch(0, smallvec![Value::Unit])
}

pub fn eval_init_circuit_data(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check_96 @ Value::Unit]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    let num_inputs = match _registry.get_type(&info.ty).unwrap() {
        CoreTypeConcrete::Circuit(CircuitTypeConcrete::Circuit(info)) => info.circuit_info.n_inputs,
        _ => todo!("{}", info.ty),
    };

    EvalAction::NormalBranch(
        0,
        smallvec![
            range_check_96,
            Value::Circuit(Vec::with_capacity(num_inputs)),
        ],
    )
}

pub fn eval_try_into_circuit_modulus(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Struct(members)]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    let [Value::BoundedInt {
        range: r0,
        value: l0,
    }, Value::BoundedInt {
        range: r1,
        value: l1,
    }, Value::BoundedInt {
        range: r2,
        value: l2,
    }, Value::BoundedInt {
        range: r3,
        value: l3,
    }]: [Value; 4] = members.try_into().unwrap()
    else {
        panic!()
    };
    assert_eq!(r0, BigInt::ZERO..(BigInt::from(1) << 96));
    assert_eq!(r1, BigInt::ZERO..(BigInt::from(1) << 96));
    assert_eq!(r2, BigInt::ZERO..(BigInt::from(1) << 96));
    assert_eq!(r3, BigInt::ZERO..(BigInt::from(1) << 96));

    let l0 = l0.to_biguint().unwrap();
    let l1 = l1.to_biguint().unwrap();
    let l2 = l2.to_biguint().unwrap();
    let l3 = l3.to_biguint().unwrap();

    let value = l0 | (l1 << 96) | (l2 << 192) | (l3 << 288);
    EvalAction::NormalBranch(0, smallvec![Value::CircuitModulus(value)])
}

pub fn eval_into_u96_guarantee(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::BoundedInt { range, value }]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };
    assert_eq!(range, BigInt::ZERO..(BigInt::from(1) << 96));

    EvalAction::NormalBranch(0, smallvec![Value::U128(value.try_into().unwrap())])
}

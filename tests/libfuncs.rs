use std::{path::Path, sync::Arc};

use cairo_lang_compiler::{compile_cairo_project_at_path, CompilerConfig};
use cairo_lang_sierra::{
    extensions::{core::CoreTypeConcrete, starknet::StarkNetTypeConcrete},
    program::{GenFunction, Program, StatementIdx},
};
use sierra_emu::{ProgramTrace, StateDump, Value, VirtualMachine};

fn run_program(path: &str, func_name: &str, args: &[Value]) -> Vec<Value> {
    let path = Path::new(path);

    let sierra_program = Arc::new(
        compile_cairo_project_at_path(
            path,
            CompilerConfig {
                replace_ids: true,
                ..Default::default()
            },
        )
        .unwrap(),
    );

    let function = find_entry_point_by_name(&sierra_program, func_name).unwrap();

    let mut vm = VirtualMachine::new(sierra_program.clone());

    let mut args = args.iter().cloned();
    let initial_gas = 1000000;

    vm.push_frame(
        function.id.clone(),
        function
            .signature
            .param_types
            .iter()
            .map(|type_id| {
                let type_info = vm.registry().get_type(type_id).unwrap();
                match type_info {
                    CoreTypeConcrete::GasBuiltin(_) => Value::U128(initial_gas),
                    CoreTypeConcrete::StarkNet(StarkNetTypeConcrete::System(_)) => Value::Unit,
                    CoreTypeConcrete::RangeCheck(_)
                    | CoreTypeConcrete::Pedersen(_)
                    | CoreTypeConcrete::Poseidon(_)
                    | CoreTypeConcrete::Bitwise(_)
                    | CoreTypeConcrete::BuiltinCosts(_)
                    | CoreTypeConcrete::SegmentArena(_) => Value::Unit,
                    _ => args.next().unwrap(),
                }
            })
            .collect::<Vec<_>>(),
    );

    let mut trace = ProgramTrace::new();

    while let Some((statement_idx, state)) = vm.step() {
        trace.push(StateDump::new(statement_idx, state));
    }

    trace
        .states
        .last()
        .unwrap()
        .items
        .values()
        .cloned()
        .collect()
}

#[test]
fn test_u32_overflow() {
    let r = run_program(
        "tests/tests/test_u32.cairo",
        "test_u32::test_u32::run_test",
        &[Value::U32(2), Value::U32(2)],
    );
    assert!(matches!(
        r[1],
        Value::Enum {
            self_ty: _,
            index: 0,
            payload: _
        }
    ));

    let r = run_program(
        "tests/tests/test_u32.cairo",
        "test_u32::test_u32::run_test",
        &[Value::U32(2), Value::U32(3)],
    );
    assert!(matches!(
        r[1],
        Value::Enum {
            self_ty: _,
            index: 1,
            payload: _
        }
    ));

    let r = run_program(
        "tests/tests/test_u32.cairo",
        "test_u32::test_u32::run_test",
        &[Value::U32(0), Value::U32(0)],
    );
    assert!(matches!(
        r[1],
        Value::Enum {
            self_ty: _,
            index: 0,
            payload: _
        }
    ));
}

pub fn find_entry_point_by_idx(
    program: &Program,
    entry_point_idx: usize,
) -> Option<&GenFunction<StatementIdx>> {
    program
        .funcs
        .iter()
        .find(|x| x.id.id == entry_point_idx as u64)
}

pub fn find_entry_point_by_name<'a>(
    program: &'a Program,
    name: &str,
) -> Option<&'a GenFunction<StatementIdx>> {
    program
        .funcs
        .iter()
        .find(|x| x.id.debug_name.as_ref().map(|x| x.as_str()) == Some(name))
}

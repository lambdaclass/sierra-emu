use std::{path::Path, sync::Arc};

use cairo_lang_compiler::{compile_cairo_project_at_path, CompilerConfig};

use crate::{create_vm, ProgramTrace, StateDump, Value};

pub fn run_program_assert_result(path: &Path, entry_point: &str, expected_output: Vec<Value>) {
    let entry_point = entry_point.parse().unwrap();

    let program = compile_cairo_project_at_path(
        path,
        CompilerConfig {
            replace_ids: true,
            ..Default::default()
        },
    )
    .unwrap();

    let mut vm = create_vm(Arc::new(program), entry_point, vec![], None).unwrap();

    let mut trace = ProgramTrace::new();

    while let Some((statement_idx, state)) = vm.step() {
        trace.push(StateDump::new(statement_idx, state));
    }

    let output = trace.states.last().unwrap();

    let result = {
        let Value::Enum {
            self_ty: _,
            index: _,
            payload,
        } = output.item().last_entry().unwrap().get().clone()
        else {
            panic!("The program does not output anything");
        };
        let Value::Struct(result) = *payload else {
            panic!();
        };

        result
    };

    assert_eq!(expected_output, result,);
}

#![cfg(test)]

use std::{fs, path::Path, sync::Arc};

use cairo_lang_compiler::{
    compile_prepared_db, db::RootDatabase, diagnostics::DiagnosticsReporter,
    project::setup_project, CompilerConfig,
};
use cairo_lang_filesystem::db::init_dev_corelib;
use cairo_lang_sierra::{
    extensions::{
        circuit::CircuitTypeConcrete, core::CoreTypeConcrete, starknet::StarkNetTypeConcrete,
    },
    program::Program,
};

use crate::{find_entry_point_by_idx, ProgramTrace, StateDump, Value, VirtualMachine};

#[macro_export]
macro_rules! load_cairo {
    ( $( $program:tt )+ ) => {
        $crate::test_utils::load_cairo_from_str(stringify!($($program)+))
    };
}

pub(crate) fn load_cairo_from_str(cairo_str: &str) -> (String, Program) {
    let mut file = tempfile::Builder::new()
        .prefix("test_")
        .suffix(".cairo")
        .tempfile()
        .unwrap();
    let mut db = RootDatabase::default();

    fs::write(&mut file, cairo_str).unwrap();

    init_dev_corelib(
        &mut db,
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("corelib/src"),
    );

    let main_crate_ids = setup_project(&mut db, file.path()).unwrap();

    let sierra_with_dbg = compile_prepared_db(
        &db,
        main_crate_ids,
        CompilerConfig {
            diagnostics_reporter: DiagnosticsReporter::stderr(),
            replace_ids: true,
            ..Default::default()
        },
    )
    .unwrap();

    let module_name = file.path().with_extension("");
    let module_name = module_name.file_name().unwrap().to_str().unwrap();
    (module_name.to_string(), sierra_with_dbg.program)
}

pub fn run_test_program(sierra_program: Program) -> Vec<Value> {
    let function = find_entry_point_by_idx(&sierra_program, 0).unwrap();

    let mut vm = VirtualMachine::new(Arc::new(sierra_program.clone()));

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
                    CoreTypeConcrete::GasBuiltin(_) => crate::Value::U128(initial_gas),
                    CoreTypeConcrete::StarkNet(StarkNetTypeConcrete::System(_)) => Value::Unit,
                    CoreTypeConcrete::RangeCheck(_)
                    | CoreTypeConcrete::RangeCheck96(_)
                    | CoreTypeConcrete::Pedersen(_)
                    | CoreTypeConcrete::Poseidon(_)
                    | CoreTypeConcrete::Bitwise(_)
                    | CoreTypeConcrete::BuiltinCosts(_)
                    | CoreTypeConcrete::SegmentArena(_)
                    | CoreTypeConcrete::Circuit(
                        CircuitTypeConcrete::AddMod(_) | CircuitTypeConcrete::MulMod(_),
                    ) => Value::Unit,
                    _ => unreachable!(),
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

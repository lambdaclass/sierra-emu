use self::args::CmdArgs;
use cairo_lang_sierra::{
    extensions::{
        circuit::CircuitTypeConcrete, core::CoreTypeConcrete, starknet::StarkNetTypeConcrete,
    },
    ProgramParser,
};
use clap::Parser;
use sierra_emu::{ProgramTrace, StateDump, Value, VirtualMachine};
use std::{
    fs::{self, File},
    io::stdout,
    sync::Arc,
};
use tracing::{debug, info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CmdArgs::parse();

    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .with_max_level(Level::TRACE)
            .finish(),
    )?;

    info!("Loading the Sierra program from disk.");
    let source_code = fs::read_to_string(args.program)?;

    info!("Parsing the Sierra program.");
    let program = Arc::new(
        ProgramParser::new()
            .parse(&source_code)
            .map_err(|e| e.to_string())?,
    );

    info!("Preparing the virtual machine.");
    let mut vm = VirtualMachine::new(program.clone());

    debug!("Pushing the entry point's frame.");
    let function = program
        .funcs
        .iter()
        .find(|f| match &args.entry_point {
            args::EntryPoint::Number(x) => f.id.id == *x,
            args::EntryPoint::String(x) => f.id.debug_name.as_deref() == Some(x.as_str()),
        })
        .unwrap();

    debug!(
        "Entry point argument types: {:?}",
        function.signature.param_types
    );
    let mut iter = args.args.into_iter();
    vm.push_frame(
        function.id.clone(),
        function
            .signature
            .param_types
            .iter()
            .map(|type_id| {
                let type_info = vm.registry().get_type(type_id).unwrap();
                match type_info {
                    CoreTypeConcrete::Felt252(_) => Value::parse_felt(&iter.next().unwrap()),
                    CoreTypeConcrete::GasBuiltin(_) => Value::U128(args.available_gas.unwrap()),
                    CoreTypeConcrete::RangeCheck(_)
                    | CoreTypeConcrete::RangeCheck96(_)
                    | CoreTypeConcrete::Bitwise(_)
                    | CoreTypeConcrete::Pedersen(_)
                    | CoreTypeConcrete::Poseidon(_)
                    | CoreTypeConcrete::SegmentArena(_)
                    | CoreTypeConcrete::Circuit(
                        CircuitTypeConcrete::AddMod(_) | CircuitTypeConcrete::MulMod(_),
                    ) => Value::Unit,
                    CoreTypeConcrete::StarkNet(inner) => match inner {
                        StarkNetTypeConcrete::System(_) => Value::Unit,
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            })
            .collect::<Vec<_>>(),
    );

    let mut trace = ProgramTrace::new();

    info!("Running the program.");
    while let Some((statement_idx, state)) = vm.step() {
        trace.push(StateDump::new(statement_idx, state));
    }

    match args.output {
        Some(path) => serde_json::to_writer(File::create(path)?, &trace)?,
        None => serde_json::to_writer(stdout().lock(), &trace)?,
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use cairo_lang_compiler::CompilerConfig;
    use cairo_lang_starknet::compile::compile_path;
    use sierra_emu::{
        find_entry_point_by_idx, ContractExecutionResult, ProgramTrace, StateDump, VirtualMachine,
    };

    #[test]
    fn test_contract() {
        let path = Path::new("programs/hello_starknet.cairo");

        let contract = compile_path(
            path,
            None,
            CompilerConfig {
                replace_ids: true,
                ..Default::default()
            },
        )
        .unwrap();

        let sierra_program = contract.extract_sierra_program().unwrap();

        let entry_point = contract.entry_points_by_type.external.first().unwrap();
        let function = find_entry_point_by_idx(&sierra_program, entry_point.function_idx).unwrap();

        let mut vm = VirtualMachine::new(sierra_program.clone().into());

        let calldata = [2.into()];
        let initial_gas = 1000000;

        vm.call_contract(function, initial_gas, calldata);

        let mut trace = ProgramTrace::new();

        while let Some((statement_idx, state)) = vm.step() {
            trace.push(StateDump::new(statement_idx, state));
        }

        // let trace_str = serde_json::to_string_pretty(&trace).unwrap();
        // std::fs::write("contract_trace.json", trace_str).unwrap();
    }

    #[test]
    fn test_contract_constructor() {
        let path = Path::new("programs/hello_starknet.cairo");

        let contract = compile_path(
            path,
            None,
            CompilerConfig {
                replace_ids: true,
                ..Default::default()
            },
        )
        .unwrap();

        let sierra_program = contract.extract_sierra_program().unwrap();

        let entry_point = contract.entry_points_by_type.constructor.first().unwrap();
        let function = find_entry_point_by_idx(&sierra_program, entry_point.function_idx).unwrap();

        let mut vm = VirtualMachine::new(sierra_program.clone().into());

        let calldata = [2.into()];
        let initial_gas = 1000000;

        vm.call_contract(function, initial_gas, calldata);

        let mut trace = ProgramTrace::new();

        while let Some((statement_idx, state)) = vm.step() {
            trace.push(StateDump::new(statement_idx, state));
        }

        assert!(!vm.syscall_handler.storage.is_empty());

        let result = ContractExecutionResult::from_trace(&trace).unwrap();
        assert!(!result.failure_flag);
        assert_eq!(result.return_values.len(), 0);
        assert_eq!(result.error_msg, None);

        // let trace_str = serde_json::to_string_pretty(&trace).unwrap();
        // std::fs::write("contract_trace.json", trace_str).unwrap();
    }
}

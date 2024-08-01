use self::{args::CmdArgs, vm::VirtualMachine};
use cairo_lang_sierra::ProgramParser;
use clap::Parser;
use dump::{ProgramTrace, StateDump};
use std::{
    fs::{self, File},
    io::stdout,
};
use tracing::{debug, info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod args;
mod dump;
mod value;
mod vm;

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
    let program = ProgramParser::new()
        .parse(&source_code)
        .map_err(|e| e.to_string())?;

    info!("Preparing the virtual machine.");
    let mut vm = VirtualMachine::new(&program);

    debug!("Pushing the entry point's frame.");
    vm.push_frame(
        &program
            .funcs
            .iter()
            .find(|f| match &args.entry_point {
                args::EntryPoint::Number(x) => f.id.id == *x,
                args::EntryPoint::String(x) => f.id.debug_name.as_deref() == Some(x.as_str()),
            })
            .unwrap()
            .id,
        [
            // TODO: Entry point argument parsing.
        ],
    );

    let mut trace = ProgramTrace::new();

    info!("Running the program.");
    while let Some((statement_idx, state)) = vm.step() {
        trace.push(StateDump::new(statement_idx, state));
    }

    match args.output {
        Some(path) => serde_json::to_writer_pretty(File::create(path)?, &trace)?,
        None => serde_json::to_writer_pretty(stdout().lock(), &trace)?,
    };

    Ok(())
}

use self::{args::CmdArgs, vm::VirtualMachine};
use cairo_lang_sierra::ProgramParser;
use clap::Parser;
use std::fs;
use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod args;
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

    let mut vm = VirtualMachine::new(program.clone());
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
        [],
    );

    while let Some(state) = vm.step() {
        println!("{state:#?}");
    }

    Ok(())
}

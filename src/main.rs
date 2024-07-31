use self::args::CmdArgs;
use clap::Parser;
use tracing::{info, Level};
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

    info!("Hello, world!");
    info!("Args: {args:?}");

    Ok(())
}

#![doc = "Command-line entry point for SSI `FastConnect` Data."]

use clap::Parser as _;

use args::Cli;
use run::run;

mod args;
mod run;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    run(Cli::parse()).await?;
    Ok(())
}

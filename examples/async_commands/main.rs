use anyhow::Error;
use clap::{Parser, Subcommand};
use cli_context::CliContext;

mod cli_context;
mod commands;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,

    /// Your API token
    #[arg(global = true, long)]
    token: Option<String>,
    // NOTE: If you add more arguments, adjust `./cli_context.rs` accordingly
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Commands related to a project
    Project(commands::project::Command),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = CliArgs::parse();
    let cli_context = CliContext::from(&cli);

    match cli.command {
        Commands::Project(cmd) => commands::project::execute(&cli_context, cmd).await?,
    }
    
    Ok(())
}
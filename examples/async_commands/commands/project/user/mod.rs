use clap::{Args, Subcommand};
use clap_nested_commands::generate_async_commands;
use crate::cli_context::CliContext;

mod add;
mod remove;

/// User commands
#[derive(Debug, Args)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

generate_async_commands!(add, remove);

use crate::cli_context::CliContext;
use clap::{Args, Subcommand};
use clap_nested_commands::generate_async_commands;

mod task;
mod user;

/// Project commands
#[derive(Debug, Args)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

generate_async_commands!(return_type = String;
    #[command(alias = "tasks")]
    task,
    #[command(alias = "users")]
    user
);

use clap::{Args, Subcommand};
use clap_nested_commands::generate_sync_commands;
use crate::cli_context::CliContext;

mod task;
mod user;

/// Project commands
#[derive(Debug, Args)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

generate_sync_commands!(return_type = String; task, user);

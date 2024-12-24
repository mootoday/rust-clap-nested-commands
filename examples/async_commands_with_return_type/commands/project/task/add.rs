use anyhow::Error;
use clap::Args;
use crate::cli_context::CliContext;

/// Add a project task
#[derive(Debug, Args)]
pub struct Command {
    /// The description of the task to add
    #[arg(long)]
    pub description: String,
}

pub async fn execute(_cli_context: &CliContext, cmd: Command) -> Result<String, Error> {
    // TODO: Wait for 500ms to illustrate an async operation
    Ok(format!("Task added: {}", cmd.description))
}

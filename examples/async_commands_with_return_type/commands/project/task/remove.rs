use anyhow::Error;
use clap::Args;
use crate::cli_context::CliContext;

/// Remove a task
#[derive(Debug, Args)]
pub struct Command {
    /// The id of the task to remove
    #[arg(long)]
    pub id: String,
}

pub async fn execute(_cli_context: &CliContext, cmd: Command) -> Result<String, Error> {
    // TODO: Wait for 500ms to illustrate an async operation
    Ok(format!("Task removed: {}", cmd.id))
}

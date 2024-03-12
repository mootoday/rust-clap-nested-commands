use anyhow::Error;
use clap::Args;
use crate::cli_context::CliContext;

/// Remove a task
#[derive(Debug, Args)]
pub struct Command {
    /// The id of the task to remove
    pub id: String,
}

pub async fn execute(_cli_context: &CliContext, _cmd: Command) -> Result<(), Error> {
    // TODO: Wait for 500ms to illustrate an async operation
    Ok(())
}

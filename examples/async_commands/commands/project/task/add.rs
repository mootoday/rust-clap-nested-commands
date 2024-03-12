use anyhow::Error;
use clap::Args;
use crate::cli_context::CliContext;

/// Add a project task
#[derive(Debug, Args)]
pub struct Command {
    /// The description of the task to add
    pub description: String,
}

pub async fn execute(_cli_context: &CliContext, _cmd: Command) -> Result<(), Error> {
    // TODO: Wait for 500ms to illustrate an async operation
    Ok(())
}

use anyhow::Error;
use clap::Args;
use crate::cli_context::CliContext;

/// Remove a user
#[derive(Debug, Args)]
pub struct Command {
    /// The email address of the user to remove
    #[arg(long)]
    pub email: String,
}

pub async fn execute(_cli_context: &CliContext, _cmd: Command) -> Result<(), Error> {
    // TODO: Wait for 500ms to illustrate an async operation
    Ok(())
}

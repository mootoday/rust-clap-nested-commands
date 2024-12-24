use anyhow::Error;
use clap::Args;
use crate::cli_context::CliContext;

/// Add a user
#[derive(Debug, Args)]
pub struct Command {
    /// The email address of the user to add
    #[arg(long)]
    pub email: String,
}

pub async fn execute(_cli_context: &CliContext, cmd: Command) -> Result<String, Error> {
    // TODO: Wait for 500ms to illustrate an async operation
    Ok(format!("User added: {}", cmd.email))
}

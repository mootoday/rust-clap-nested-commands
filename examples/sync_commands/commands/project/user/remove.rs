use clap::Args;
use crate::cli_context::CliContext;

/// Remove a user
#[derive(Debug, Args)]
pub struct Command {
    /// The email address of the user to remove
    #[arg(long)]
    pub email: String,
}

pub fn execute(_cli_context: &CliContext, _cmd: Command) {
    // TODO: Print something meaningful
}

use clap::Args;
use crate::cli_context::CliContext;

/// Add a user
#[derive(Debug, Args)]
pub struct Command {
    /// The email address of the user to add
    pub email: String,
}

pub fn execute(_cli_context: &CliContext, _cmd: Command) {
    // TODO: Print something meaningful
}

use clap::Args;
use crate::cli_context::CliContext;

/// Add a project task
#[derive(Debug, Args)]
pub struct Command {
    /// The description of the task to add
    #[arg(long)]
    pub description: String,
}

pub fn execute(_cli_context: &CliContext, _cmd: Command) {
    // TODO: Print something meaningful
}

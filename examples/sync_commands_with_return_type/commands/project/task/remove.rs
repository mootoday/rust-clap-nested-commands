use clap::Args;
use crate::cli_context::CliContext;

/// Remove a task
#[derive(Debug, Args)]
pub struct Command {
    /// The id of the task to remove
    #[arg(long)]
    pub id: String,
}

pub fn execute(_cli_context: &CliContext, cmd: Command) -> String {
    // TODO: Print something meaningful
    format!("Task removed: {}", cmd.id)
}

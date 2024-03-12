#[macro_export]
macro_rules! generate_sync_commands {
    ($($module:ident),*) => (
      paste::paste! {
        #[derive(Debug, Subcommand)]
        pub enum Commands {
            $(
                [<$module:camel>]($module::Command),
        )*
        }

        pub fn execute(cli_context: &CliContext, cmd: Command) {
            match cmd.command {
              $(
                Commands::[<$module:camel>](cmd) => $module::execute(cli_context, cmd),
              )*
            }
        }
      }
    );
}

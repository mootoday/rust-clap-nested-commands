#[macro_export]
macro_rules! generate_async_commands {
    ($($module:ident),*) => (
      paste::paste! {
        #[derive(Debug, Subcommand)]
        pub enum Commands {
            $(
                [<$module:camel>]($module::Command),
        )*
        }

        pub async fn execute(cli_context: &CliContext, cmd: Command) -> Result<(), anyhow::Error> {
            match cmd.command {
              $(
                Commands::[<$module:camel>](cmd) => $module::execute(cli_context, cmd).await,
              )*
            }
        }
      }
    );
}

#[macro_export]
macro_rules! generate_async_commands {
    ($($(#[$attr:meta])* $module:ident),*) => (
        generate_async_commands!(return_type = (); $($(#[$attr])* $module),*);
    );
    (return_type = $return_type:ty; $($(#[$attr:meta])* $module:ident),*) => (
      paste::paste! {
        #[derive(Debug, Subcommand)]
        pub enum Commands {
            $(
                $(#[$attr])*
                [<$module:camel>]($module::Command),
            )*
        }

        pub async fn execute(cli_context: &CliContext, cmd: Command) -> Result<$return_type, anyhow::Error> {
            match cmd.command {
              $(
                Commands::[<$module:camel>](cmd) => $module::execute(cli_context, cmd).await,
              )*
            }
        }
      }
    );
}
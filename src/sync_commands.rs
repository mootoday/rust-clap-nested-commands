#[macro_export]
macro_rules! generate_sync_commands {
    ($($(#[$attr:meta])* $module:ident),*) => (
        generate_sync_commands!(return_type = (); $($(#[$attr])* $module),*);
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

        pub fn execute(cli_context: &CliContext, cmd: Command) -> $return_type {
            match cmd.command {
              $(
                Commands::[<$module:camel>](cmd) => $module::execute(cli_context, cmd),
              )*
            }
        }
      }
    );
}

use crate::CliArgs;

#[derive(Debug)]
pub struct CliContext {
    pub token: Option<String>,
}

impl<'a> From<&'a CliArgs> for CliContext {
    fn from(cli: &'a CliArgs) -> Self {
        CliContext {
            token: cli.token.clone(),
        }
    }
}

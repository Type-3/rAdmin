use crate::ServerError;

pub trait CliModule {
    fn arg(&self) -> Option<String>;
    fn app(&self) -> Option<clap::App<'static, 'static>>;
    fn handle<'a>(&self, _: Option<&clap::ArgMatches<'a>>) -> Result<(), ServerError>;
}

pub struct NullCliModule;

impl CliModule for NullCliModule {
    fn arg(&self) -> Option<String> {
        None
    }
    fn app(&self) -> Option<clap::App<'static, 'static>> {
        None
    }
    fn handle<'a>(&self, _: Option<&clap::ArgMatches<'a>>) -> Result<(), ServerError> {
        Ok(())
    }
}

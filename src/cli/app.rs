use clap::{App, ArgMatches};

use super::Database;
use crate::modules::CliModule;
use crate::ServerError;

pub struct CliApp<'a>(&'a crate::Application, Database<'a>);

impl<'a> CliApp<'a> {
    pub fn new(app: &'a crate::Application) -> CliApp<'a> {
        CliApp(&app, Database::new(&app))
    }
}

impl<'a> CliModule for CliApp<'a> {
    fn arg(&self) -> Option<String> {
        unreachable!()
    }

    fn app(&self) -> Option<App<'static, 'static>> {
        let mut app = App::new(self.0.name)
            .version(self.0.version)
            .about(self.0.description)
            .author(self.0.author)
            .subcommand(self.1.app().unwrap());
        for module in &self.0.modules.0 {
            if let Some(cmd) = module.cli().app() {
                app = app.subcommand(cmd);
            }
        }
        Some(app)
    }

    fn handle<'b>(&self, matches: Option<&ArgMatches<'b>>) -> Result<(), ServerError> {
        dotenv::dotenv().unwrap();
        let matches = matches.unwrap();
        if matches.subcommand.is_none() {
            Err(crate::rocket_factory(None, &self.0.modules)?
                .launch()
                .into())
        } else {
            let (cmd, matches) = matches.subcommand();
            if cmd == self.1.arg().unwrap() {
                self.1.handle(matches)?;
            }
            for module in &self.0.modules.0 {
                let cli = (*module).cli();
                if let Some(arg) = cli.arg() {
                    if arg == cmd {
                        cli.handle(matches)?;
                    }
                }
            }
            Ok(())
        }
    }
}

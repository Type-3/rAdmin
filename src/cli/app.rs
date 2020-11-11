use clap::{App, ArgMatches};

use super::{Database, Storage};
use crate::modules::CliModule;
use crate::ServerError;

pub struct CliApp<'a> {
    app: &'a crate::Application,
    database: Database<'a>,
    storage: Storage<'a>
}

impl<'a> CliApp<'a> {
    pub fn new(app: &'a crate::Application) -> CliApp<'a> {
        CliApp {
            app: &app,
            database: Database::new(&app),
            storage: Storage::new(&app)
        }
    }
}

impl<'a> CliModule for CliApp<'a> {
    fn arg(&self) -> Option<String> {
        unreachable!()
    }

    fn app(&self) -> Option<App<'static, 'static>> {
        let mut app = App::new(self.app.name)
            .version(self.app.version)
            .about(self.app.description)
            .author(self.app.author)
            .subcommand(self.storage.app().unwrap())
            .subcommand(self.database.app().unwrap());
        for module in &self.app.modules.0 {
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
            Err(crate::rocket_factory(None, &self.app.modules)?
                .launch()
                .into())
        } else {
            let (cmd, matches) = matches.subcommand();
            if cmd == self.database.arg().unwrap() {
                self.database.handle(matches)?;
            } else if cmd == self.storage.arg().unwrap() {
                self.storage.handle(matches)?;
            }
            for module in &self.app.modules.0 {
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

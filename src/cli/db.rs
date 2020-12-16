use crate::modules::CliModule;
use crate::ServerError;
use termion::{color, style};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub struct Database<'a>(&'a crate::Application);

impl<'a> Database<'a> {
    pub fn new(modules: &'a crate::Application) -> Database {
        Database(modules)
    }
}

impl<'a> CliModule for Database<'a> {
    fn arg(&self) -> Option<String> {
        Some("db".into())
    }

    fn app(&self) -> Option<App<'static, 'static>> {
        let mut seed_cmd = SubCommand::with_name("seed");
        for module in &self.0.modules.0 {
            if let Some(args) = (*module).database().seeder().args() {
                for arg in args {
                    seed_cmd = seed_cmd.arg(arg);
                }
            }
        }
        Some(
            SubCommand::with_name("db")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("init")
                        .arg(Arg::with_name("reset").short("r").long("reset"))
                        .arg(Arg::with_name("seed").short("s").long("seed")),
                )
                .subcommand(seed_cmd)
                .subcommand(SubCommand::with_name("reset")),
        )
    }

    fn handle(&self, matches: Option<&ArgMatches>) -> Result<(), ServerError> {
        match matches {
            None => unreachable!(),
            Some(matches) => match matches.subcommand() {
                ("init", matches) => self.handle_init_command(matches)?,
                ("seed", matches) => self.handle_seed_command(matches)?,
                ("reset", matches) => self.handle_reset_command(matches)?,
                _ => {}
            },
        }
        Ok(())
    }
}

impl<'a> Database<'a> {
    fn handle_init_command(&self, args: Option<&ArgMatches>) -> Result<(), ServerError> {
        let reset = args.map(|item| item.is_present("reset")).unwrap_or(false);
        let seed = args.map(|item| item.is_present("seed")).unwrap_or(false);
        let conn = crate::establish_connection()?;
        if reset {
            self.handle_reset_command(None)?;
        }
        crate::database::migrate(&conn)?;
        println!("{}Running database migration's{}", color::Fg(color::Green), style::Reset);
        for module in &self.0.modules.0 {
            println!(
                "    {}->{} {}{}({}){}",
                color::Fg(color::Blue),
                style::Reset,
                style::Italic,
                module.identifier(),
                module.version(),
                style::Reset
            );
            (*module).database().run_migrations(&conn)?;
        }
        if seed {
            self.handle_seed_command(None)?;
        }
        Ok(())
    }

    fn handle_reset_command(&self, _: Option<&ArgMatches>) -> Result<(), ServerError> {
        use diesel::RunQueryDsl;
        let conn = crate::establish_connection()?;
        diesel::select(crate::database::radmin_reset_database).execute(&conn)?;
        Ok(())
    }

    fn handle_seed_command(&self, matches: Option<&ArgMatches>) -> Result<(), ServerError> {
        let conn = crate::database::establish_connection()?;
        println!("{}Running database seeder's{}", color::Fg(color::Green), style::Reset);
        for module in &self.0.modules.0 {
            println!(
                "    {}->{} {}{}({}){}",
                color::Fg(color::Blue),
                style::Reset,
                style::Italic,
                module.identifier(),
                module.version(),
                style::Reset
            );
            match (*module).database().seeder().seed(matches, &conn) {
                Ok(_) => {}
                Err(err) => {
                    println!(
                        "{}{}({}){}: {}{:?}{}",
                        color::Fg(color::Red),
                        module.identifier(),
                        module.version(),
                        style::Reset,
                        style::Italic,
                        err,
                        style::Reset
                    );
                }
            }
        }
        Ok(())
    }
}

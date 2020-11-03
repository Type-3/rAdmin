use crate::modules::CliModule;
use crate::ServerError;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use cli_table::format::{CellFormat, Color, TableFormat};
use cli_table::{Cell, Row, Table};

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
                .subcommand(SubCommand::with_name("reset"))
                .subcommand(
                    SubCommand::with_name("search")
                        .arg(Arg::with_name("query").index(1).required(true)),
                ),
        )
    }

    fn handle(&self, matches: Option<&ArgMatches>) -> Result<(), ServerError> {
        match matches {
            None => unreachable!(),
            Some(matches) => match matches.subcommand() {
                ("init", matches) => self.handle_init_command(matches)?,
                ("seed", matches) => self.handle_seed_command(matches)?,
                ("reset", matches) => self.handle_reset_command(matches)?,
                ("search", matches) => self.handle_search_command(matches)?,
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
        crate::database::migrate_with_output(&conn, &mut std::io::stdout())?;
        for module in &self.0.modules.0 {
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
        for module in &self.0.modules.0 {
            println!(
                "Running {}({}) seeder",
                module.identifier(),
                module.version()
            );
            match (*module).database().seeder().seed(matches, &conn) {
                Ok(_) => {},
                Err(err) => {
                    println!("{}({}): {:?}", module.identifier(), module.version(), err);
                }
            }
        }
        Ok(())
    }

    fn handle_search_command(&self, matches: Option<&ArgMatches>) -> Result<(), ServerError> {
        let query = matches.unwrap().value_of("query").unwrap();
        let search_args = self.0.modules.get_search_arguments()?;
        let results = crate::controllers::GlobalSearch::new(search_args).query(query)?;
        let mut rows = vec![Row::new(vec![
            Cell::new("Context", header_format()),
            Cell::new("Fields", header_format()),
            Cell::new("Id", header_format()),
        ])];
        for result in &results {
            let ctx = Cell::new(&result.context, Default::default());
            let id = Cell::new(&result.id, Default::default());
            let mut fields = String::new();
            for (dex, (field, value)) in result.fields.iter().enumerate() {
                if dex > 0 {
                    fields.push('\n');
                }
                fields.push_str(&format!("{}: {}", field, value));
            }
            rows.push(Row::new(vec![
                ctx,
                Cell::new(&fields, Default::default()),
                id,
            ]));
        }
        Table::new(rows, table_format())
            .unwrap()
            .print_stdout()
            .unwrap();
        Ok(())
    }
}

fn header_format() -> CellFormat {
    CellFormat::builder()
        .foreground_color(Some(Color::Yellow))
        .bold(true)
        .build()
}

fn table_format() -> TableFormat {
    TableFormat::default().foreground(Color::Red)
}

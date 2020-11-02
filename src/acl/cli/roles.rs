use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use super::tables::RolesTable;
use crate::acl::factories::RoleFactory;
use crate::acl::schema::roles;
use crate::cli::Table;
use crate::modules::CliModule;
use crate::ServerError;

pub struct Roles;

impl CliModule for Roles {
    fn arg(&self) -> Option<String> {
        Some("roles".into())
    }
    fn app(&self) -> Option<App<'static, 'static>> {
        Some(
            SubCommand::with_name("roles")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("list")
                        .arg(Arg::with_name("page").long("page").takes_value(true))
                        .arg(
                            Arg::with_name("per_page")
                                .long("per-page")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("add")
                        .arg(
                            Arg::with_name("name")
                                .index(1)
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("label")
                                .short("l")
                                .long("label")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("description")
                                .short("d")
                                .long("description")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("rm").arg(
                        Arg::with_name("id")
                            .index(1)
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
    }

    fn handle(&self, matches: Option<&ArgMatches>) -> Result<(), ServerError> {
        match matches.unwrap().subcommand() {
            ("list", matches) => RolesTable::default().display(matches),
            ("add", Some(matches)) => handle_add_function(matches)?,
            ("rm", Some(matches)) => handle_rm_function(matches)?,
            _ => {}
        }
        Ok(())
    }
}

fn handle_add_function(matches: &ArgMatches) -> Result<(), ServerError> {
    let name = value_t_or_exit!(matches, "name", String);
    let label = matches.value_of("label").map(ToString::to_string);
    let description = matches.value_of("description").map(ToString::to_string);

    let db = crate::establish_connection()?;
    RoleFactory::default()
        .name(name)
        .label(label)
        .description(description)
        .insert(&db);
    Ok(())
}

fn handle_rm_function(matches: &ArgMatches) -> Result<(), ServerError> {
    let id = value_t_or_exit!(matches, "id", Uuid);
    let db = crate::establish_connection().unwrap();
    diesel::delete(roles::table.find(id)).execute(&db)?;
    Ok(())
}

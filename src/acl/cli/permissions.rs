use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use super::tables::PermissionsTable;
use crate::acl::factories::PermissionFactory;
use crate::acl::schema::permissions;
use crate::cli::Table;
use crate::modules::CliModule;
use crate::ServerError;

pub struct Permissions;

impl CliModule for Permissions {
    fn arg(&self) -> Option<String> {
        Some("permissions".into())
    }
    fn app(&self) -> Option<App<'static, 'static>> {
        Some(
            SubCommand::with_name("permissions")
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
            ("list", matches) => PermissionsTable::default().display(matches),
            ("add", Some(matches)) => handle_add_command(matches)?,
            ("rm", Some(matches)) => handle_rm_command(matches)?,
            _ => {}
        }
        Ok(())
    }
}

fn handle_add_command(matches: &ArgMatches) -> Result<(), ServerError> {
    let name = value_t_or_exit!(matches, "name", String);
    let label = matches.value_of("label").map(ToString::to_string);
    let description = matches.value_of("description").map(ToString::to_string);

    let db = crate::establish_connection()?;
    PermissionFactory::default()
        .name(name)
        .label(label)
        .description(description)
        .insert(&db);
    Ok(())
}

fn handle_rm_command(matches: &ArgMatches) -> Result<(), ServerError> {
    let id = value_t_or_exit!(matches, "id", Uuid);
    let db = crate::establish_connection()?;
    diesel::delete(permissions::table.find(id)).execute(&db)?;
    Ok(())
}

use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use super::tables::AccountsTable;
use crate::acl::factories::AccountFactory;
use crate::acl::schema::accounts;
use crate::cli::Table;
use crate::modules::CliModule;
use crate::types::PasswordType;
use crate::ServerError;

pub struct Accounts;

impl CliModule for Accounts {
    fn arg(&self) -> Option<String> {
        Some("accounts".into())
    }
    fn app(&self) -> Option<App<'static, 'static>> {
        Some(
            SubCommand::with_name("accounts")
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
                            Arg::with_name("username")
                                .short("u")
                                .long("username")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("pass")
                                .short("p")
                                .long("pass")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("type")
                                .short("t")
                                .long("type")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("rm").arg(
                        Arg::with_name("id")
                            .short("i")
                            .long("id")
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
    }

    fn handle<'a>(&self, matches: Option<&ArgMatches<'a>>) -> Result<(), ServerError> {
        match matches.unwrap().subcommand() {
            ("list", matches) => AccountsTable::default().display(matches),
            ("add", Some(matches)) => handle_add_command(matches)?,
            ("rm", Some(matches)) => handle_rm_command(matches)?,
            _ => {}
        }
        Ok(())
    }
}

fn handle_add_command(matches: &ArgMatches) -> Result<(), ServerError> {
    let username = value_t_or_exit!(matches, "username", String);
    let pass = value_t_or_exit!(matches, "pass", String);
    let ty = match matches.value_of("type") {
        None => PasswordType::Argon2,
        Some("argon2") => PasswordType::Argon2,
        Some("bcrypt") => PasswordType::Bcrypt,
        Some(_) => panic!("Invalid password hash type"),
    };
    let db = crate::establish_connection().unwrap();
    AccountFactory::default()
        .username(username)
        .set_password_with_hash(ty, &pass)?
        .insert(&db);
    Ok(())
}

fn handle_rm_command(matches: &ArgMatches) -> Result<(), ServerError> {
    let id = value_t_or_exit!(matches, "id", Uuid);
    let db = crate::establish_connection().unwrap();
    diesel::delete(accounts::table.find(id)).execute(&db)?;
    Ok(())
}

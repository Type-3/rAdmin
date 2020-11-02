use crate::modules::CliModule;
use crate::ServerError;
use clap::{App, AppSettings, ArgMatches, SubCommand};

mod accounts;
mod permissions;
mod roles;
mod tables;

pub struct AclCli;

impl CliModule for AclCli {
    fn arg(&self) -> Option<String> {
        Some("acl".into())
    }

    fn app(&self) -> Option<App<'static, 'static>> {
        Some(
            SubCommand::with_name("acl")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(accounts::Accounts.app().unwrap())
                .subcommand(roles::Roles.app().unwrap())
                .subcommand(permissions::Permissions.app().unwrap()),
        )
    }

    fn handle(&self, matches: Option<&ArgMatches>) -> Result<(), ServerError> {
        match matches {
            None => unreachable!(),
            Some(matches) => match matches.subcommand() {
                ("accounts", matches) => accounts::Accounts.handle(matches)?,
                ("roles", matches) => roles::Roles.handle(matches)?,
                ("permissions", matches) => permissions::Permissions.handle(matches)?,
                _ => {}
            },
        }
        Ok(())
    }
}

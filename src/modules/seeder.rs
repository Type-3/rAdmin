use crate::ServerError;
use clap::{Arg, ArgMatches};
use diesel::PgConnection;

pub trait Seeder {
    fn args(&self) -> Option<Vec<Arg<'static, 'static>>>;
    fn seed(&self, matches: Option<&ArgMatches>, _: &PgConnection) -> Result<(), ServerError>;
}

pub struct NullSeeder;

impl Seeder for NullSeeder {
    fn args(&self) -> Option<Vec<Arg<'static, 'static>>> {
        None
    }
    fn seed(&self, _: Option<&ArgMatches>, _: &PgConnection) -> Result<(), ServerError> {
        Ok(())
    }
}

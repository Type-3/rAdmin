use diesel::PgConnection;

use super::{NullSeeder, Seeder};
use crate::ServerError;

pub trait DatabaseModule {
    fn seeder(&self) -> Box<dyn Seeder> {
        Box::new(NullSeeder)
    }
    fn run_migrations(&self, _: &PgConnection) -> Result<(), ServerError>;
}

pub struct NullDatabaseModule;

impl DatabaseModule for NullDatabaseModule {
    fn run_migrations(&self, _: &PgConnection) -> Result<(), ServerError> {
        Ok(())
    }
}

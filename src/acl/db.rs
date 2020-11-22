use diesel::PgConnection;

use super::AclSeeder;
use crate::modules::DatabaseModule;
use crate::modules::Seeder;
use crate::ServerError;

pub struct AclDbMod;

impl DatabaseModule for AclDbMod {
    fn seeder(&self) -> Box<dyn Seeder> {
        Box::new(AclSeeder)
    }
    /// The migrations for the Acl are located with the "Server" migrations and are run
    /// automatically.
    fn run_migrations(&self, _: &PgConnection) -> Result<(), ServerError> {
        Ok(())
    }
}

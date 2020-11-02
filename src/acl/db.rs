use diesel::PgConnection;

use super::ServerSeeder;
use crate::controllers::SearchConfig;
use crate::modules::DatabaseModule;
use crate::modules::Seeder;
use crate::ServerError;

pub struct AclDbMod;

impl DatabaseModule for AclDbMod {
    fn seeder(&self) -> Box<dyn Seeder> {
        Box::new(ServerSeeder)
    }
    /// The migrations for the Acl are located with the "Server" migrations and are run
    /// automatically.
    fn run_migrations(&self, _: &PgConnection) -> Result<(), ServerError> {
        Ok(())
    }

    fn search(&self) -> Result<SearchConfig, ServerError> {
        Ok(vec![
            (
                "roles".into(),
                "Role".into(),
                "admin.roles.list".into(),
                vec!["name".into(), "label".into(), "description".into()],
            ),
            (
                "permissions".into(),
                "Permission".into(),
                "admin.permissions.list".into(),
                vec!["name".into(), "label".into(), "description".into()],
            ),
            (
                "accounts".into(),
                "Account".into(),
                "admin.accounts.list".into(),
                vec!["email".into(), "username".into()],
            ),
        ])
    }
}

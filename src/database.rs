use diesel::{Connection, PgConnection};
use diesel_migrations::embed_migrations;
use rocket_contrib::database;

use crate::ServerError;

embed_migrations!("migrations");

pub use self::embedded_migrations::run as migrate;
pub use self::embedded_migrations::run_with_output as migrate_with_output;

#[database("radmin_db")]
pub struct DbConnection(pub PgConnection);

impl AsRef<PgConnection> for DbConnection {
    fn as_ref(&self) -> &PgConnection {
        &self.0
    }
}

use diesel::*;

no_arg_sql_function!(
    radmin_reset_database,
    (),
    "Represents the SQL radmin_reset_database() function"
);

pub fn establish_connection() -> Result<PgConnection, ServerError> {
    Ok(PgConnection::establish(&std::env::var("DATABASE_URL")?)?)
}

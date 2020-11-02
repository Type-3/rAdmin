//! The Test client is included in the crate instead of in the tests module to
//! allow the admin tests to share it.

use derive_more::AsRef;
use diesel::helper_types::Find;
use diesel::query_dsl::methods::{FindDsl, LoadQuery};
use diesel::{Connection, PgConnection, RunQueryDsl};
use std::env;
use uuid::Uuid;

use crate::modules::Modules;
use crate::ServerError;

#[derive(AsRef)]
pub struct DbClient(PgConnection);

impl DbClient {
    pub fn new(modules: Option<&Modules>) -> Result<DbClient, ServerError> {
        dotenv::from_filename("testing.env").unwrap();
        let var = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&var)?;
        crate::database::migrate(&conn).unwrap();
        if let Some(modules) = modules {
            for module in &modules.0 {
                (*module).database().run_migrations(&conn)?;
            }
        }
        Ok(DbClient(conn))
    }

    /// This function will check that the supplied table has a row with the primary key `id`
    /// it then assets that this row matches the given model.
    pub fn assert_table_has<Model, Table>(&self, table: Table, id: Uuid, model: Model)
    where
        Table: FindDsl<Uuid>,
        Model: std::fmt::Debug + PartialEq,
        Find<Table, Uuid>: LoadQuery<PgConnection, Model>,
    {
        assert_eq!(table.find(id).load::<Model>(&self.0).unwrap(), vec![model]);
    }

    /// This function asserts that the given `table` does not contain a row with
    /// primary key `id`.
    pub fn assert_table_missing<Model, Table>(&self, table: Table, id: Uuid)
    where
        Table: FindDsl<Uuid>,
        Model: std::fmt::Debug + PartialEq,
        Find<Table, Uuid>: LoadQuery<PgConnection, Model>,
    {
        assert_eq!(table.find(id).load::<Model>(&self.0).unwrap(), vec![]);
    }
}

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::schema::accounts;
use crate::types::PasswordType;
use crate::ServerError;

#[derive(Debug, PartialEq, Clone, Queryable, Serialize, Deserialize, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub password_type: PasswordType,
    #[serde(skip)]
    pub password_hash: Vec<u8>,
    #[serde(skip)]
    pub password_salt: Vec<u8>,
    #[serde(skip)]
    pub auth_token: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub avatar: Option<Uuid>,
    pub roles: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Account {
    pub fn from_auth_token(token: &str, conn: &PgConnection) -> Result<Account, ServerError> {
        Ok(accounts::table
            .filter(accounts::auth_token.eq(Some(token)))
            .first(conn)?)
    }
}

use cli_table::Cell;

impl Into<Vec<Cell>> for Account {
    fn into(self) -> Vec<Cell> {
        vec![
            Cell::new(&self.id, Default::default()),
            Cell::new(&self.email, Default::default()),
            Cell::new(&self.username, Default::default()),
            Cell::new(&self.created_at, Default::default()),
            Cell::new(&self.updated_at, Default::default()),
        ]
    }
}

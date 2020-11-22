use chrono::NaiveDateTime;
use diesel::{PgConnection, RunQueryDsl};
use fake::faker::internet::en::{SafeEmail, Username};
use fake::Fake;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::models::Account;
use crate::acl::schema::accounts;
use crate::acl::Auth;
use crate::types::PasswordType;
use crate::ServerError;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Insertable)]
#[table_name = "accounts"]
pub struct AccountFactory {
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub username: String,
    pub password_type: PasswordType,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub avatar: Option<Uuid>,
    pub roles: Vec<Uuid>,
}

impl AccountFactory {
    pub fn email<S: Into<String>>(mut self, email: S) -> AccountFactory {
        self.email = email.into();
        self
    }

    pub fn email_verified_at(mut self, date: NaiveDateTime) -> AccountFactory {
        self.email_verified_at = Some(date);
        self
    }

    pub fn username<S: Into<String>>(mut self, username: S) -> AccountFactory {
        self.username = username.into();
        self
    }

    pub fn avatar<S: Into<Option<Uuid>>>(mut self, avatar: S) -> AccountFactory {
        self.avatar = avatar.into();
        self
    }

    pub fn roles(mut self, roles: Vec<Uuid>) -> AccountFactory {
        self.roles = roles;
        self
    }

    pub fn set_password_with_hash(
        mut self,
        hash: PasswordType,
        pass: &str,
    ) -> Result<AccountFactory, ServerError> {
        self.password_type = hash;
        self.password_salt = Auth::generate_salt();
        self.password_hash = Auth::hash_password(pass, &self.password_salt, hash)?;
        Ok(self)
    }

    pub fn set_password(self, pass: &str) -> Result<AccountFactory, ServerError> {
        self.set_password_with_hash(PasswordType::default(), pass)
    }

    pub fn insert(self, conn: &PgConnection) -> Account {
        diesel::insert_into(crate::acl::schema::accounts::table)
            .values(&self)
            .get_result(conn)
            .expect(&format!("Failed to insert into database: {:?}", &self))
    }
}

impl Default for AccountFactory {
    fn default() -> AccountFactory {
        AccountFactory {
            email: SafeEmail().fake(),
            email_verified_at: None,
            username: Username().fake(),
            password_type: PasswordType::Argon2,
            password_hash: vec![],
            password_salt: vec![],
            avatar: None,
            roles: vec![],
        }
    }
}

use chrono::NaiveDateTime;
use diesel::PgConnection;
use diesel_factories::Factory;
use fake::faker::internet::en::{SafeEmail, Username};
use fake::Fake;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::models::Account;
use crate::acl::Auth;
use crate::types::PasswordType;
use crate::ServerError;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Factory)]
#[factory(model = "Account", table = "crate::acl::schema::accounts", id = "Uuid")]
pub struct AccountFactory {
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub username: String,
    pub password_type: PasswordType,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub avatar: Option<Uuid>
}

impl AccountFactory {
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
        Factory::insert(self, conn)
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
            avatar: None
        }
    }
}

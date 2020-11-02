use chrono::NaiveDateTime;
use diesel::PgConnection;
use diesel_factories::Factory;
use fake::faker::internet::en::{SafeEmail, Username};
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::models::Account;
use crate::acl::Auth;
use crate::types::PasswordType;
use crate::ServerError;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Factory, Dummy)]
#[factory(model = "Account", table = "crate::acl::schema::accounts", id = "Uuid")]
pub struct AccountFactory {
    #[dummy(faker = "SafeEmail()")]
    pub email: String,
    #[dummy("")]
    pub email_verified_at: Option<NaiveDateTime>,
    #[dummy(faker = "Username()")]
    pub username: String,
    pub password_type: PasswordType,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
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
        Faker.fake()
    }
}

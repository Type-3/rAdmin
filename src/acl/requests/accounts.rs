use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::Validate;
use validator::ValidationErrors;
use validator_derive::Validate;

use crate::acl::models::Account;
use crate::acl::Auth;
use crate::traits::{Fillable, Validatable};
use crate::ServerError;

#[derive(Serialize, Deserialize, Debug, Validate, TypedBuilder)]
pub struct AccountRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4))]
    pub username: String,
    #[validate(
        must_match(
            other = "password_confirm",
            message = "Password and Password Confirmation Must Match"
        ),
        length(min = 4, message = "Password to short"),
        length(max = 25, message = "Password to long")
    )]
    pub password: Option<String>,
    pub password_confirm: Option<String>,
    pub roles: Vec<Uuid>,
    pub avatar: Option<Uuid>,
}

impl Validatable for AccountRequest {
    fn validate(self) -> Result<AccountRequest, ValidationErrors> {
        Validate::validate(&self)?;
        Ok(self)
    }
}

impl Fillable<Account> for AccountRequest {
    fn fill(self, account: &mut Account) -> Result<(), ServerError> {
        let config = crate::config::get_rocket_config(None)?;
        account.email = self.email;
        account.username = self.username;
        account.password_salt = Auth::generate_salt();
        account.avatar = self.avatar;
        account.roles = self.roles;
        if let Some(pass) = self.password {
            account.password_hash = Auth::hash_password(
                &pass,
                &account.password_salt,
                Auth::get_password_hash(&config),
            )?;
        }
        Ok(())
    }
}

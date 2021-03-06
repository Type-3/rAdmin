use diesel::{PgConnection, SaveChangesDsl};
use serde::Deserialize;
use uuid::Uuid;

use crate::acl::factories::AccountFactory;
use crate::acl::models::Account;
use crate::acl::requests::AccountRequest;
use crate::traits::Submitable;
use crate::types::PasswordType;
use crate::ServerError;

#[derive(Deserialize, Debug)]
pub struct AccountCreateForm {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub roles: Vec<String>,
    pub avatar: Option<Uuid>,
    #[serde(skip)]
    pub pw_type: PasswordType,
}

impl AccountCreateForm {
    pub fn from_req(pw_type: PasswordType, req: AccountRequest) -> AccountCreateForm {
        AccountCreateForm {
            email: req.email,
            username: req.username,
            password: req.password.unwrap(),
            password_confirm: req.password_confirm.unwrap(),
            roles: req.roles,
            avatar: req.avatar,
            pw_type,
        }
    }
}

impl From<AccountRequest> for AccountCreateForm {
    fn from(req: AccountRequest) -> AccountCreateForm {
        AccountCreateForm {
            email: req.email,
            username: req.username,
            password: req.password.unwrap(),
            password_confirm: req.password_confirm.unwrap(),
            roles: req.roles,
            avatar: req.avatar,
            pw_type: PasswordType::Argon2,
        }
    }
}

impl Submitable for AccountCreateForm {
    fn submit(self, conn: &PgConnection) -> Result<(), ServerError> {
        let mut account = AccountFactory::default()
            .email(self.email)
            .username(self.username)
            .set_password_with_hash(self.pw_type, &self.password)?
            .avatar(self.avatar)
            .roles(self.roles)
            .insert(conn);
        account.avatar = self.avatar;
        account.save_changes::<Account>(conn)?;
        Ok(())
    }
}

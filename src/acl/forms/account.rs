use diesel::PgConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::acl::factories::AccountFactory;
use crate::acl::requests::AccountRequest;
use crate::acl::traits::{HasPermissions, HasRoles};
use crate::traits::Submitable;
use crate::types::PasswordType;
use crate::ServerError;

#[derive(Deserialize, Debug)]
pub struct AccountCreateForm {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub roles: Vec<Uuid>,
    pub permissions: Vec<Uuid>,
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
            permissions: req.permissions,
            pw_type,
        }
    }
}

impl Submitable for AccountCreateForm {
    fn submit(self, conn: &PgConnection) -> Result<(), ServerError> {
        let account = AccountFactory::default()
            .username(self.username)
            .set_password_with_hash(self.pw_type, &self.password)?
            .insert(conn);
        account.sync_roles(&self.roles, conn)?;
        account.sync_permissions(&self.permissions, conn)?;
        Ok(())
    }
}

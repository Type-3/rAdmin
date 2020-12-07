use crate::acl::models::Account;
use crate::crud::CrudDelete;
use crate::roles::AdminRole;

pub struct AccountsDelete;

impl CrudDelete<Account> for AccountsDelete {
    type Role = AdminRole;
}

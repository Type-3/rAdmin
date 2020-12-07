use crate::acl::forms::AccountCreateForm;
use crate::acl::requests::AccountRequest;
use crate::crud::CrudStore;
use crate::roles::AdminRole;

pub struct AccountsStore;

impl CrudStore for AccountsStore {
    type Role = AdminRole;
    type Form = AccountCreateForm;
    type Request = AccountRequest;
}

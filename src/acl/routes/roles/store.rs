use crate::acl::forms::RoleCreateForm;
use crate::acl::requests::RoleRequest;
use crate::crud::CrudStore;
use crate::roles::AdminRole;

pub struct RolesStore;

impl CrudStore for RolesStore {
    type Role = AdminRole;
    type Form = RoleCreateForm;
    type Request = RoleRequest;
}

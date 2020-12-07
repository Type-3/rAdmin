use crate::acl::models::Role;
use crate::crud::CrudDelete;
use crate::roles::AdminRole;

pub struct RolesDelete;

impl CrudDelete<Role> for RolesDelete {
    type Role = AdminRole;
}

use radmin_macros::Permission;

pub trait PermissionDef {
    const NAME: &'static str;
}

#[derive(Permission)]
#[name("admin.roles.list")]
pub struct RolesList;

#[derive(Permission)]
#[name("admin.roles.modify")]
pub struct RolesModify;

#[derive(Permission)]
#[name("admin.roles.delete")]
pub struct RolesDelete;

#[derive(Permission)]
#[name("admin.permissions.list")]
pub struct PermissionsList;

#[derive(Permission)]
#[name("admin.permissions.modify")]
pub struct PermissionsModify;

#[derive(Permission)]
#[name("admin.permissions.delete")]
pub struct PermissionsDelete;

#[derive(Permission)]
#[name("admin.accounts.list")]
pub struct AccountsList;

#[derive(Permission)]
#[name("admin.accounts.modify")]
pub struct AccountsModify;

#[derive(Permission)]
#[name("admin.accounts.delete")]
pub struct AccountsDelete;

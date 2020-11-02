use crate::acl::models::{Account, Permission, Role};
use crate::cli::Table;

#[derive(Default)]
pub struct AccountsTable;

impl Table<Account> for AccountsTable {
    const HEADERS: &'static [&'static str] =
        &["Id", "Email", "Username", "Created At", "Updated At"];
}

#[derive(Default)]
pub struct PermissionsTable;

impl Table<Permission> for PermissionsTable {
    const HEADERS: &'static [&'static str] = &["Id", "Name", "Label", "Description"];
}

#[derive(Default)]
pub struct RolesTable;

impl Table<Role> for RolesTable {
    const HEADERS: &'static [&'static str] = &["Id", "Name", "Label", "Description"];
}

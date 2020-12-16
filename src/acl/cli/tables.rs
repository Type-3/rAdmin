use crate::acl::models::{Account, Role};
use crate::cli::Table;

#[derive(Default)]
pub struct AccountsTable;

impl Table<Account> for AccountsTable {
    const HEADERS: &'static [&'static str] =
        &["Id", "Email", "Username", "Roles", "Created At", "Updated At"];
}

#[derive(Default)]
pub struct RolesTable;

impl Table<Role> for RolesTable {
    const HEADERS: &'static [&'static str] = &["Name", "Label", "Roles", "Description"];
}

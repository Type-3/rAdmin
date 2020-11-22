use crate::cli::Table;
use crate::acl::models::{Account, Role};

#[derive(Default)]
pub struct AccountsTable;

impl Table<Account> for AccountsTable {
    const HEADERS: &'static [&'static str] =
        &["Id", "Email", "Username", "Created At", "Updated At"];
}

#[derive(Default)]
pub struct RolesTable;

impl Table<Role> for RolesTable {
    const HEADERS: &'static [&'static str] = &["Id", "Name", "Label", "Roles", "Description"];
}

use clap::{Arg, ArgMatches};
use diesel::PgConnection;

use crate::acl::factories::PermissionFactory;
use crate::traits::Submitable;
use crate::modules::Seeder;
use crate::ServerError;

const SERVER_PERMISSIONS: &[(&str, &str, &str)] = &[
    (
        "admin.accounts.list",
        "List Account",
        "Can List all current login accounts.",
    ),
    (
        "admin.accounts.modify",
        "Accounts Modify",
        "Can modify login accounts.",
    ),
    (
        "admin.accounts.delete",
        "Accounts Delete",
        "Can delete login accounts.",
    ),
    (
        "admin.roles.list",
        "List Roles",
        "Can list all current login roles.",
    ),
    (
        "admin.roles.modify",
        "Roles Modify",
        "Can modify login roles.",
    ),
    (
        "admin.roles.delete",
        "Roles Delete",
        "Can delete login roles.",
    ),
    (
        "admin.permissions.list",
        "List Permissions",
        "Can list all current login permissions.",
    ),
    (
        "admin.permissions.modify",
        "Permissions Modify",
        "Can modify login permissions.",
    ),
    (
        "admin.permissions.delete",
        "Permissions Delete",
        "Can delete login permissions.",
    ),
];

pub struct AclSeeder;

impl Seeder for AclSeeder {
    fn args(&self) -> Option<Vec<Arg<'static, 'static>>> {
        None
    }

    fn seed(&self, _matches: Option<&ArgMatches>, conn: &PgConnection) -> Result<(), ServerError> {
        use std::fs::File;
        use crate::acl::forms::{RoleCreateForm, PermissionCreateForm};
        insert_default_permissions(conn)?;

        if let Ok(file) = File::open("seeds/roles.json") {
            let roles: Vec<RoleCreateForm> = serde_json::from_reader(file)?;
            for role in roles {
                role.submit(conn)?;
            }
        }
        if let Ok(file) = File::open("seeds/permissions.json") {
            let permissions: Vec<PermissionCreateForm> = serde_json::from_reader(file)?;
            for permission in permissions {
                permission.submit(conn)?;
            }
        }
        Ok(())
    }
}


fn insert_default_permissions(conn: &PgConnection) -> Result<(), ServerError> {
    for (name, label, description) in SERVER_PERMISSIONS.iter() {
        PermissionFactory::default()
            .name(name.to_string())
            .label(label.to_string())
            .description(description.to_string())
            .insert(conn);
    }
    Ok(())
}

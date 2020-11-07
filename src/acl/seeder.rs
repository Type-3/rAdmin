use clap::{Arg, ArgMatches};
use diesel::{PgConnection, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::{Value, Map, json};
use crate::acl::factories::PermissionFactory;
use crate::traits::Submitable;
use crate::modules::Seeder;
use crate::ServerError;
use uuid::Uuid;

use std::path::PathBuf;

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
        use crate::acl::forms::{RoleCreateForm, PermissionCreateForm, AccountCreateForm};
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
        if let Ok(file) = File::open("seeds/accounts.json") {
            use crate::acl::schema::roles;
            let accounts: Vec<Map<String, Value>> = serde_json::from_reader(file)?;
            for mut account in accounts.into_iter() {
                if account.contains_key("roles") {
                    account["roles"] = json!(account.get("roles")
                                              .expect("Json does not contain the `roles` field")
                                              .as_array()
                                              .expect("`roles` field is not JSON array")
                                              .iter()
                                              .map(|item| {
                        roles::table.select(roles::id)
                            .filter(roles::name.eq(item.as_str().unwrap()))
                            .first::<uuid::Uuid>(conn)
                            .expect(&format!("Role named `{}` does not exists", item.as_str().unwrap()))
                    })
                    .collect::<Vec<uuid::Uuid>>());
                }
                account.insert("password_confirm".into(), account["password"].clone());
                if !account.contains_key("roles") {
                    account.insert("roles".into(), json!(Vec::<Value>::new()));
                }
                if !account.contains_key("permissions") {
                    account.insert("permissions".into(), json!(Vec::<Value>::new()));
                }
                if account.contains_key("avatar") {
                    let mut seed_path = PathBuf::from("seeds/avatars/");
                    let mut out_path = PathBuf::from("data/avatars/");
                    let _avatar = account.remove("avatar").unwrap();
                    let avatar = _avatar.as_str().unwrap();
                    seed_path.push(&avatar);
                    if seed_path.exists() {
                        let id = Uuid::new_v4();
                        out_path.push(format!("{}.png", id));
                        std::fs::copy(&seed_path, &out_path)?;
                        account.insert("avatar".into(), json!(id));
                    }
                }
                let account: AccountCreateForm = serde_json::from_value(json!(account))?;
                account.submit(conn)?;
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

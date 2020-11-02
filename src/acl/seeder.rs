use clap::{value_t, Arg, ArgMatches};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::acl::factories::{AccountFactory, PermissionFactory, RoleFactory};
use crate::acl::models::{Permission, Role};
use crate::acl::schema::{permissions, roles};
use crate::acl::traits::{HasPermissions, HasRoles};
use crate::modules::Seeder;
use crate::types::PasswordType;
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

const SERVER_ROLES: &[(&str, &str, &str)] = &[("admin", "Admin", "Admin Users Can Do Anything.")];

pub struct ServerSeeder;

impl Seeder for ServerSeeder {
    fn args(&self) -> Option<Vec<Arg<'static, 'static>>> {
        Some(vec![
            Arg::with_name("no-default").long("no-default"),
            Arg::with_name("seed_accounts")
                .long("accounts")
                .takes_value(true),
            Arg::with_name("seed_permissions")
                .long("permissions")
                .takes_value(true),
            Arg::with_name("seed_roles").long("roles").takes_value(true),
            Arg::with_name("seed_admin_password")
                .long("admin-password")
                .takes_value(true),
            Arg::with_name("seed_admin_name")
                .long("admin-name")
                .takes_value(true),
            Arg::with_name("seed_admin_email")
                .long("admin_email")
                .takes_value(true),
            Arg::with_name("seed_init"),
        ])
    }

    fn seed(&self, matches: Option<&ArgMatches>, conn: &PgConnection) -> Result<(), ServerError> {
        if let Some(false) = matches.map(|item| item.is_present("no-default")) {
            run_init_seeders(conn, matches)?;
        }

        if let Some(matches) = matches {
            if let Ok(accounts) = value_t!(matches.value_of("seed_accounts"), usize) {
                for _ in 1..accounts {
                    AccountFactory::default()
                        .set_password("password")?
                        .insert(&conn);
                }
            }
            if let Ok(roles) = value_t!(matches.value_of("seed_roles"), usize) {
                for _ in 1..roles {
                    RoleFactory::default().insert(&conn);
                }
            }
            if let Ok(permissions) = value_t!(matches.value_of("seed_permissions"), usize) {
                for _ in 1..permissions {
                    PermissionFactory::default().insert(&conn);
                }
            }
        }

        Ok(())
    }
}

pub fn run_init_seeders(
    conn: &PgConnection,
    matches: Option<&ArgMatches>,
) -> Result<(), ServerError> {
    insert_default_permissions(conn)?;
    insert_default_roles(conn)?;
    assign_all_permissions_to_admin(conn)?;

    let (name, email, pass) = if let Some(matches) = matches {
        let admin_name = value_t!(matches.value_of("seed_admin_name"), String)
            .unwrap_or_else(|_| "admin".to_string());
        let admin_email = value_t!(matches.value_of("seed_admin_email"), String)
            .unwrap_or_else(|_| "admin@admin.com".to_string());
        let admin_pass = value_t!(matches.value_of("seed_admin_password"), String)
            .unwrap_or_else(|_| "admin".to_string());
        (admin_name, admin_email, admin_pass)
    } else {
        ("admin".into(), "admin@admin.com".into(), "admin".into())
    };
    let admin_account = crate::acl::factories::AccountFactory::default()
        .username(name)
        .email(email)
        .email_verified_at(chrono::Utc::now().naive_utc())
        .set_password_with_hash(PasswordType::Argon2, &pass)?
        .insert(conn);
    admin_account.assign_role_name("admin", conn)?;
    Ok(())
}

fn insert_default_roles(conn: &PgConnection) -> Result<(), ServerError> {
    for (name, label, description) in SERVER_ROLES.iter() {
        RoleFactory::default()
            .name(name.to_string())
            .label(label.to_string())
            .description(description.to_string())
            .insert(conn);
    }
    Ok(())
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

fn assign_all_permissions_to_admin(conn: &PgConnection) -> Result<(), ServerError> {
    let permissions = permissions::table.load::<Permission>(conn)?;
    let role: Role = roles::table.filter(roles::name.eq("admin")).first(conn)?;
    for permission in permissions {
        role.assign_permission(&permission, conn)?;
    }
    Ok(())
}

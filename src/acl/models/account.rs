use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::models::{Permission, Role};
use crate::acl::schema::{account_permissions, account_roles, accounts, permissions, roles};
use crate::acl::traits::{HasPermissions, HasRoles};
use crate::types::PasswordType;
use crate::ServerError;

#[derive(Debug, PartialEq, Clone, Queryable, Serialize, Deserialize, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub password_type: PasswordType,
    #[serde(skip)]
    pub password_hash: Vec<u8>,
    #[serde(skip)]
    pub password_salt: Vec<u8>,
    #[serde(skip)]
    pub auth_token: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub avatar: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl HasPermissions for Account {
    fn permissions(&self, conn: &PgConnection) -> Result<Vec<Permission>, ServerError> {
        let account_id = account_permissions::account_id.eq(self.id);
        let mut permissions: Vec<Permission> = account_permissions::table
            .filter(account_id)
            .inner_join(permissions::table)
            .select(permissions::all_columns)
            .load(conn)?;
        for role in self.roles(conn)? {
            for permission in role.permissions(conn)? {
                if !permissions.contains(&permission) {
                    permissions.push(permission);
                }
            }
        }
        Ok(permissions)
    }

    fn permission_ids(&self, conn: &PgConnection) -> Result<Vec<Uuid>, ServerError> {
        let account_id = account_permissions::account_id.eq(self.id);
        let mut permissions: Vec<Uuid> = account_permissions::table
            .filter(account_id)
            .inner_join(permissions::table)
            .select(permissions::id)
            .load(conn)?;
        for permission in self.permissions(conn)? {
            if !permissions.contains(&permission.id) {
                permissions.push(permission.id);
            }
        }
        Ok(permissions)
    }

    fn permission_names(&self, conn: &PgConnection) -> Result<Vec<String>, ServerError> {
        let account_id = account_permissions::account_id.eq(self.id);
        let mut permissions: Vec<String> = account_permissions::table
            .filter(account_id)
            .inner_join(permissions::table)
            .select(permissions::name)
            .load(conn)?;
        for role in self.roles(conn)? {
            for permission in role.permission_names(conn)? {
                if !permissions.contains(&permission) {
                    permissions.push(permission);
                }
            }
        }
        Ok(permissions)
    }

    fn assign_permission_id(
        &self,
        permission: Uuid,
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        let permission_id = account_permissions::permission_id.eq(permission);
        let account_id = account_permissions::account_id.eq(self.id);
        diesel::insert_into(account_permissions::table)
            .values((permission_id, account_id))
            .execute(conn)?;
        Ok(())
    }

    fn assign_permission_ids(
        &self,
        permissions: &[Uuid],
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        for permission in permissions {
            self.assign_permission_id(*permission, conn)?;
        }
        Ok(())
    }

    fn assign_permission_name(
        &self,
        permission: &str,
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        let permission: Uuid = permissions::table
            .filter(permissions::name.eq(permission))
            .select(permissions::id)
            .first(conn)?;
        self.assign_permission_id(permission, conn)
    }

    fn sync_permissions(
        &self,
        permissions: &[Uuid],
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        diesel::delete(
            account_permissions::table.filter(account_permissions::account_id.eq(self.id)),
        )
        .execute(conn)?;
        self.assign_permission_ids(permissions, conn)?;
        Ok(())
    }

    fn has_permission_id(
        &self,
        permission: Uuid,
        conn: &PgConnection,
    ) -> Result<bool, ServerError> {
        Ok(self.permission_ids(conn)?.contains(&permission))
    }

    fn has_permission_name(&self, name: &str, conn: &PgConnection) -> Result<bool, ServerError> {
        let permission: Uuid = permissions::table
            .filter(permissions::name.eq(name))
            .select(permissions::id)
            .first(conn)?;
        self.has_permission_id(permission, conn)
    }

    fn revoke_permission_id(
        &self,
        permission: Uuid,
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        let permission_id = account_permissions::permission_id.eq(permission);
        let account_id = account_permissions::account_id.eq(self.id);
        let statement = account_permissions::table
            .filter(permission_id)
            .filter(account_id);
        diesel::delete(statement).execute(conn)?;
        Ok(())
    }

    fn revoke_permission_name(&self, name: &str, conn: &PgConnection) -> Result<(), ServerError> {
        let permission: Uuid = permissions::table
            .filter(permissions::name.eq(name))
            .select(permissions::id)
            .first(conn)?;
        self.revoke_permission_id(permission, conn)
    }
}

impl Account {
    pub fn from_auth_token(token: &str, conn: &PgConnection) -> Result<Account, ServerError> {
        Ok(accounts::table
            .filter(accounts::auth_token.eq(Some(token)))
            .first(conn)?)
    }
}

use cli_table::Cell;

impl Into<Vec<Cell>> for Account {
    fn into(self) -> Vec<Cell> {
        vec![
            Cell::new(&self.id, Default::default()),
            Cell::new(&self.email, Default::default()),
            Cell::new(&self.username, Default::default()),
            Cell::new(&self.created_at, Default::default()),
            Cell::new(&self.updated_at, Default::default()),
        ]
    }
}

impl HasRoles for Account {
    fn roles(&self, conn: &PgConnection) -> Result<Vec<Role>, ServerError> {
        let account_id = account_roles::account_id.eq(self.id);
        Ok(account_roles::table
            .filter(account_id)
            .inner_join(roles::table)
            .select(roles::all_columns)
            .load(conn)?)
    }

    fn role_ids(&self, conn: &PgConnection) -> Result<Vec<Uuid>, ServerError> {
        let account_id = account_roles::account_id.eq(self.id);
        Ok(account_roles::table
            .filter(account_id)
            .inner_join(roles::table)
            .select(roles::id)
            .load(conn)?)
    }

    fn role_names(&self, conn: &PgConnection) -> Result<Vec<String>, ServerError> {
        let account_id = account_roles::account_id.eq(self.id);
        Ok(account_roles::table
            .filter(account_id)
            .inner_join(roles::table)
            .select(roles::name)
            .load(conn)?)
    }

    fn assign_role_id(&self, role: Uuid, conn: &PgConnection) -> Result<(), ServerError> {
        let role_id = account_roles::role_id.eq(role);
        let account_id = account_roles::account_id.eq(self.id);
        diesel::insert_into(account_roles::table)
            .values((role_id, account_id))
            .execute(conn)?;
        Ok(())
    }

    fn assign_role_ids(&self, roles: &[Uuid], conn: &PgConnection) -> Result<(), ServerError> {
        for role in roles {
            self.assign_role_id(*role, conn)?;
        }
        Ok(())
    }

    fn assign_role_name(&self, role: &str, conn: &PgConnection) -> Result<(), ServerError> {
        let role: Uuid = roles::table
            .filter(roles::name.eq(role))
            .select(roles::id)
            .first(conn)?;
        self.assign_role_id(role, conn)
    }

    fn sync_roles(&self, roles: &[Uuid], conn: &PgConnection) -> Result<(), ServerError> {
        diesel::delete(account_roles::table.filter(account_roles::account_id.eq(self.id)))
            .execute(conn)?;
        self.assign_role_ids(roles, conn)?;
        Ok(())
    }

    fn has_role_id(&self, role: Uuid, conn: &PgConnection) -> Result<bool, ServerError> {
        let role_id = account_roles::role_id.eq(role);
        let account_id = account_roles::account_id.eq(self.id);
        let count: i64 = account_roles::table
            .filter(role_id)
            .filter(account_id)
            .count()
            .get_result(conn)?;
        Ok(count == 1)
    }

    fn has_role_name(&self, name: &str, conn: &PgConnection) -> Result<bool, ServerError> {
        let role: Uuid = roles::table
            .filter(roles::name.eq(name))
            .select(roles::id)
            .first(conn)?;
        self.has_role_id(role, conn)
    }

    fn revoke_role_id(&self, role: Uuid, conn: &PgConnection) -> Result<(), ServerError> {
        let role_id = account_roles::role_id.eq(role);
        let account_id = account_roles::account_id.eq(self.id);
        let statement = account_roles::table.filter(role_id).filter(account_id);
        diesel::delete(statement).execute(conn)?;
        Ok(())
    }
    fn revoke_role_name(&self, name: &str, conn: &PgConnection) -> Result<(), ServerError> {
        let role: Uuid = roles::table
            .filter(roles::name.eq(name))
            .select(roles::id)
            .first(conn)?;
        let role_id = account_roles::role_id.eq(role);
        let account_id = account_roles::account_id.eq(self.id);
        let statement = account_roles::table.filter(role_id).filter(account_id);
        diesel::delete(statement).execute(conn)?;
        Ok(())
    }

    fn is_super_role(&self, conn: &PgConnection) -> Result<bool, ServerError> {
        for role in self.roles(conn)? {
            if role.is_super {
                return Ok(true);
            }
        }
        return Ok(false);
    }
}

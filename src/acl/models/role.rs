use chrono::NaiveDateTime;
use diesel::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::models::Permission;
use crate::acl::schema::{permissions, role_permissions, roles};
use crate::acl::traits::HasPermissions;
use crate::ServerError;

#[derive(Debug, PartialEq, Clone, Identifiable, Serialize, Deserialize, Queryable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl HasPermissions for Role {
    fn permissions(&self, conn: &PgConnection) -> Result<Vec<Permission>, ServerError> {
        let role_id = role_permissions::role_id.eq(self.id);
        Ok(role_permissions::table
            .filter(role_id)
            .inner_join(permissions::table)
            .select(permissions::all_columns)
            .load(conn)?)
    }

    fn permission_ids(&self, conn: &PgConnection) -> Result<Vec<Uuid>, ServerError> {
        let role_id = role_permissions::role_id.eq(self.id);
        Ok(role_permissions::table
            .filter(role_id)
            .inner_join(permissions::table)
            .select(permissions::id)
            .load(conn)?)
    }

    fn permission_names(&self, conn: &PgConnection) -> Result<Vec<String>, ServerError> {
        let role_id = role_permissions::role_id.eq(self.id);
        Ok(role_permissions::table
            .filter(role_id)
            .inner_join(permissions::table)
            .select(permissions::name)
            .load(conn)?)
    }

    fn assign_permission_id(
        &self,
        permission: Uuid,
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        let permission_id = role_permissions::permission_id.eq(permission);
        let role_id = role_permissions::role_id.eq(self.id);
        diesel::insert_into(role_permissions::table)
            .values((permission_id, role_id))
            .execute(conn)?;
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

    fn sync_permissions(
        &self,
        permissions: &[Uuid],
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        diesel::delete(role_permissions::table.filter(role_permissions::role_id.eq(self.id)))
            .execute(conn)?;
        self.assign_permission_ids(permissions, conn)?;
        Ok(())
    }

    fn has_permission_id(
        &self,
        permission: Uuid,
        conn: &PgConnection,
    ) -> Result<bool, ServerError> {
        let permission_id = role_permissions::permission_id.eq(permission);
        let role_id = role_permissions::role_id.eq(self.id);
        let count: i64 = role_permissions::table
            .filter(permission_id)
            .filter(role_id)
            .count()
            .get_result(conn)?;
        Ok(count == 1)
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
        let permission_id = role_permissions::permission_id.eq(permission);
        let role_id = role_permissions::role_id.eq(self.id);
        let statement = role_permissions::table
            .filter(permission_id)
            .filter(role_id);
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

use cli_table::Cell;

impl Into<Vec<Cell>> for Role {
    fn into(self) -> Vec<Cell> {
        let label = self.label.unwrap_or_else(|| "None".into());
        let description = self.description.unwrap_or_else(|| "None".into());
        vec![
            Cell::new(&self.id, Default::default()),
            Cell::new(&self.name, Default::default()),
            Cell::new(&label, Default::default()),
            Cell::new(&description, Default::default()),
        ]
    }
}

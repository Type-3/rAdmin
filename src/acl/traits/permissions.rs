use diesel::PgConnection;
use uuid::Uuid;

use crate::acl::models::Permission;
use crate::ServerError;

pub trait HasPermissions {
    fn permissions(&self, conn: &PgConnection) -> Result<Vec<Permission>, ServerError>;
    fn permission_ids(&self, conn: &PgConnection) -> Result<Vec<Uuid>, ServerError>;
    fn permission_names(&self, conn: &PgConnection) -> Result<Vec<String>, ServerError>;
    fn assign_permission_id(
        &self,
        permission: Uuid,
        conn: &PgConnection,
    ) -> Result<(), ServerError>;
    fn assign_permission_ids(
        &self,
        permission: &[Uuid],
        conn: &PgConnection,
    ) -> Result<(), ServerError>;
    fn assign_permission_name(
        &self,
        permission: &str,
        conn: &PgConnection,
    ) -> Result<(), ServerError>;
    fn sync_permissions(&self, roles: &[Uuid], conn: &PgConnection) -> Result<(), ServerError>;
    fn has_permission_id(&self, permission: Uuid, conn: &PgConnection)
        -> Result<bool, ServerError>;
    fn has_permission_name(
        &self,
        permission: &str,
        conn: &PgConnection,
    ) -> Result<bool, ServerError>;
    fn revoke_permission_id(
        &self,
        permission: Uuid,
        conn: &PgConnection,
    ) -> Result<(), ServerError>;
    fn revoke_permission_name(
        &self,
        permission: &str,
        conn: &PgConnection,
    ) -> Result<(), ServerError>;

    fn assign_permission(
        &self,
        permission: &Permission,
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        self.assign_permission_id(permission.id, conn)
    }

    fn has_permission(
        &self,
        permission: &Permission,
        conn: &PgConnection,
    ) -> Result<bool, ServerError> {
        self.has_permission_id(permission.id, conn)
    }

    fn revoke_permission(
        &self,
        permission: &Permission,
        conn: &PgConnection,
    ) -> Result<(), ServerError> {
        self.revoke_permission_id(permission.id, conn)
    }
}

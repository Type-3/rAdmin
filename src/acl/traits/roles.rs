use diesel::PgConnection;
use uuid::Uuid;

use crate::acl::models::Role;
use crate::ServerError;

pub trait HasRoles {
    fn roles(&self, conn: &PgConnection) -> Result<Vec<Role>, ServerError>;
    fn role_ids(&self, conn: &PgConnection) -> Result<Vec<Uuid>, ServerError>;
    fn role_names(&self, conn: &PgConnection) -> Result<Vec<String>, ServerError>;
    fn assign_role_id(&self, role: Uuid, conn: &PgConnection) -> Result<(), ServerError>;
    fn assign_role_ids(&self, role: &[Uuid], conn: &PgConnection) -> Result<(), ServerError>;
    fn assign_role_name(&self, role: &str, conn: &PgConnection) -> Result<(), ServerError>;
    fn sync_roles(&self, roles: &[Uuid], conn: &PgConnection) -> Result<(), ServerError>;
    fn has_role_id(&self, role: Uuid, conn: &PgConnection) -> Result<bool, ServerError>;
    fn has_role_name(&self, role: &str, conn: &PgConnection) -> Result<bool, ServerError>;
    fn revoke_role_id(&self, role: Uuid, conn: &PgConnection) -> Result<(), ServerError>;
    fn revoke_role_name(&self, role: &str, conn: &PgConnection) -> Result<(), ServerError>;

    fn assign_role(&self, role: &Role, conn: &PgConnection) -> Result<(), ServerError> {
        self.assign_role_id(role.id, conn)
    }

    fn has_role(&self, role: &Role, conn: &PgConnection) -> Result<bool, ServerError> {
        self.has_role_id(role.id, conn)
    }

    fn revoke_role(&self, role: &Role, conn: &PgConnection) -> Result<(), ServerError> {
        self.revoke_role_id(role.id, conn)
    }
}

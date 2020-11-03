use crate::ServerError;
use diesel::PgConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::acl::factories::RoleFactory;
use crate::acl::requests::RoleRequest;
use crate::acl::traits::HasPermissions;
use crate::traits::Submitable;

#[radmin_macros::from_similar(RoleRequest)]
#[derive(Deserialize, Debug)]
pub struct RoleCreateForm {
    pub name: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub permissions: Vec<Uuid>,
    pub is_super: bool
}

impl Submitable for RoleCreateForm {
    fn submit(self, conn: &PgConnection) -> Result<(), ServerError> {
        let role = RoleFactory::default()
            .name(self.name)
            .label(self.label)
            .description(self.description)
            .is_super(self.is_super)
            .insert(&conn);
        role.sync_permissions(&self.permissions, conn)?;
        Ok(())
    }
}

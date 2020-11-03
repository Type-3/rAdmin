use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};
use validator_derive::Validate;

use crate::acl::models::Role;
use crate::acl::traits::HasPermissions;
use crate::traits::{Fillable, Validatable};
use crate::ServerError;

#[derive(Serialize, Deserialize, PartialEq, Debug, Validate, TypedBuilder)]
pub struct RoleRequest {
    #[validate(
        length(min = 4, message = "Name is to short"),
        length(max = 256, message = "Name is to long")
    )]
    pub name: String,
    #[validate(
        length(min = 4, message = "Label is to short"),
        length(max = 256, message = "Label is to long")
    )]
    pub label: Option<String>,
    #[validate(
        length(min = 4, message = "Description is to short"),
        length(max = 256, message = "Description is to long")
    )]
    pub description: Option<String>,
    pub permissions: Vec<Uuid>,
    pub is_super: bool
}

impl Validatable for RoleRequest {
    fn validate(self) -> Result<RoleRequest, ValidationErrors> {
        Validate::validate(&self)?;
        Ok(self)
    }
}

impl Fillable<Role> for RoleRequest {
    fn fill(self, role: &mut Role, conn: &PgConnection) -> Result<(), ServerError> {
        role.name = self.name;
        role.label = self.label;
        role.description = self.description;
        role.is_super = self.is_super;
        role.sync_permissions(&self.permissions, conn)?;
        Ok(())
    }
}

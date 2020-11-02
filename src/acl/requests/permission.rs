use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationErrors};
use validator_derive::Validate;

use crate::acl::models::Permission;
use crate::traits::{Fillable, Validatable};
use crate::ServerError;

#[derive(Serialize, Deserialize, PartialEq, Debug, Validate, TypedBuilder)]
pub struct PermissionRequest {
    #[builder(setter(into))]
    #[validate(
        length(min = 4, message = "Name is to short"),
        length(max = 256, message = "Name is to long")
    )]
    pub name: String,
    #[builder(setter(into, strip_option))]
    #[validate(
        length(min = 4, message = "Label is to short"),
        length(max = 256, message = "Label is to long")
    )]
    pub label: Option<String>,
    #[builder(setter(into, strip_option))]
    #[validate(
        length(min = 4, message = "Description is to short"),
        length(max = 256, message = "Description is to long")
    )]
    pub description: Option<String>,
}

impl Validatable for PermissionRequest {
    fn validate(self) -> Result<PermissionRequest, ValidationErrors> {
        Validate::validate(&self)?;
        Ok(self)
    }
}

impl Fillable<Permission> for PermissionRequest {
    fn fill(self, perm: &mut Permission, _: &PgConnection) -> Result<(), ServerError> {
        perm.name = self.name;
        perm.description = self.description;
        perm.label = self.label;
        Ok(())
    }
}

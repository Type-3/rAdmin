use rocket::post;
use rocket_contrib::json::Json;

use crate::acl::forms::RoleCreateForm;
use crate::acl::guards::HasPermission;
use crate::permissions::RolesModify;
use crate::traits::Submitable;
use crate::traits::Validatable;
use crate::{ApiResponse, DbConnection, ServerError};

#[post("/create", data = "<role>", format = "application/json; charset=UTF-8")]
pub fn store(
    _perm: HasPermission<RolesModify>,
    role: Json<crate::acl::requests::RoleRequest>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    let req = role.into_inner().validate()?;
    let form = RoleCreateForm::from(req);
    form.submit(db.as_ref())?;
    Ok(ApiResponse::ok())
}

use rocket::post;
use rocket_contrib::json::Json;

use crate::acl::forms::PermissionCreateForm;
use crate::acl::guards::HasPermission;
use crate::permissions::PermissionsModify;
use crate::traits::Submitable;
use crate::traits::Validatable;
use crate::{ApiResponse, DbConnection, ServerError};

#[post(
    "/create",
    data = "<permission>",
    format = "application/json; charset=UTF-8"
)]
pub fn store(
    _perm: HasPermission<PermissionsModify>,
    permission: Json<crate::acl::requests::PermissionRequest>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    let req = permission.into_inner().validate()?;
    let form = PermissionCreateForm::from(req);
    form.submit(db.as_ref())?;
    Ok(ApiResponse::ok())
}

use std::str::FromStr;

use diesel::{QueryDsl, RunQueryDsl, SaveChangesDsl};
use rocket::post;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

use crate::acl::guards::HasPermission;
use crate::acl::models::Permission;
use crate::acl::schema::permissions;
use crate::permissions::PermissionsModify;
use crate::traits::{Fillable, Validatable};
use crate::{ApiResponse, DbConnection, ServerError};

#[post(
    "/<permission_id>",
    data = "<form>",
    format = "application/json; charset=UTF-8"
)]
pub fn update(
    _perm: HasPermission<PermissionsModify>,
    permission_id: Uuid,
    form: Json<crate::acl::requests::PermissionRequest>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    let req = form.into_inner().validate()?;
    let uuid: ::uuid::Uuid = ::uuid::Uuid::from_str(&permission_id.to_string()).unwrap();
    let mut permission: Permission = permissions::table.find(uuid).first(db.as_ref())?;
    req.fill(&mut permission, db.as_ref())?;
    permission.save_changes::<Permission>(db.as_ref())?;
    Ok(ApiResponse::ok())
}

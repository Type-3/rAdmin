use crate::acl::guards::HasPermission;
use crate::acl::models::Permission;
use crate::acl::schema::permissions;
use crate::permissions::PermissionsDelete;
use crate::{ApiResponse, DbConnection, ServerError};
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use rocket::delete;
use rocket_contrib::uuid::Uuid;

use std::str::FromStr;

#[delete("/<permission_id>", format = "application/json; charset=UTF-8")]
pub fn delete(
    permission_id: Uuid,
    _perm: HasPermission<PermissionsDelete>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    // Convert from Rocket Uuid Type to uuid crate Uuid
    let uuid: ::uuid::Uuid = ::uuid::Uuid::from_str(&permission_id.to_string()).unwrap();
    let permission: Permission = permissions::table
        .find(uuid)
        .first::<Permission>(db.as_ref())?;
    diesel::delete(&permission).execute(db.as_ref())?;
    Ok(ApiResponse::ok())
}

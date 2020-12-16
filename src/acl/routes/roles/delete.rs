use diesel::QueryDsl;
use diesel::RunQueryDsl;
use rocket::{delete, get};
use rocket::response::{Flash, Redirect};

use crate::acl::guards::HasRole;
use crate::acl::schema::roles;
use crate::roles::AdminRole;
use crate::{ApiResponse, DbConnection, ServerError};

#[delete("/<role_id>", format = "application/json; charset=UTF-8")]
pub fn api_delete(
    role_id: String,
    _perm: HasRole<AdminRole>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    // Convert from Rocket Uuid Type to uuid crate Uuid
    let role = roles::table.find(role_id);
    diesel::delete(role).execute(db.as_ref())?;
    Ok(ApiResponse::ok())
}

#[get("/<role_id>")]
pub fn form_delete(
    role_id: String,
    _perm: HasRole<AdminRole>,
    db: DbConnection,
) -> Result<Flash<Redirect>, ServerError> {
    // Convert from Rocket Uuid Type to uuid crate Uuid
    let role = roles::table.find(&role_id);
    diesel::delete(role).execute(db.as_ref())?;
    Ok(Flash::success(Redirect::to("/admin/roles"), format!("role {} removed", role_id)))
}

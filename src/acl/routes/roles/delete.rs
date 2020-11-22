use std::str::FromStr;

use diesel::QueryDsl;
use diesel::RunQueryDsl;
use rocket::delete;
use rocket_contrib::uuid::Uuid;

use crate::acl::guards::HasRole;
use crate::acl::models::Role;
use crate::acl::schema::roles;
use crate::roles::AdminRole;
use crate::{ApiResponse, DbConnection, ServerError};

#[delete("/<role_id>", format = "application/json; charset=UTF-8")]
pub fn delete(
    role_id: Uuid,
    _perm: HasRole<AdminRole>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    // Convert from Rocket Uuid Type to uuid crate Uuid
    let uuid: ::uuid::Uuid = ::uuid::Uuid::from_str(&role_id.to_string()).unwrap();
    let role: Role = roles::table.find(uuid).first::<Role>(db.as_ref())?;
    diesel::delete(&role).execute(db.as_ref())?;
    Ok(ApiResponse::ok())
}

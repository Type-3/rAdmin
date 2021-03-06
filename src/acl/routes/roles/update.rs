use diesel::{QueryDsl, RunQueryDsl, SaveChangesDsl};
use rocket::post;
use rocket_contrib::json::Json;

use crate::acl::guards::HasRole;
use crate::acl::models::Role;
use crate::acl::schema::roles;
use crate::roles::AdminRole;
use crate::traits::{Fillable, Validatable};
use crate::{ApiResponse, DbConnection, ServerError};

#[post(
    "/<role_id>",
    data = "<form>",
    format = "application/json; charset=UTF-8"
)]
pub fn update(
    _perm: HasRole<AdminRole>,
    role_id: String,
    form: Json<crate::acl::requests::RoleRequest>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    let req = form.into_inner().validate()?;
    let mut role: Role = roles::table.find::<String>(role_id).first(db.as_ref())?;
    req.fill(&mut role)?;
    role.save_changes::<Role>(db.as_ref())?;
//    role.save_changes::<Role>(db.as_ref())?;
    Ok(ApiResponse::ok())
}

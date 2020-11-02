use diesel::RunQueryDsl;
use rocket::get;
use serde_json::json;

use crate::acl::guards::HasPermission;
use crate::acl::models::Permission;
use crate::acl::schema::permissions;
use crate::permissions::PermissionsList;
use crate::{ApiResponse, DbConnection, ServerError};

#[get("/selectOptions", format = "application/json; charset=UTF-8")]
pub fn options(
    _perm: HasPermission<PermissionsList>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    let options: Vec<serde_json::Value> = permissions::table
        .load(db.as_ref())?
        .into_iter()
        .map(|item: Permission| {
            let text = item.label.unwrap_or(item.name);
            json!({
                "id": item.id,
                "text": text
            })
        })
        .collect();
    let select_opts = json!({ "options": options });
    Ok(ApiResponse::ok().data(select_opts))
}

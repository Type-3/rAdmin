use diesel::RunQueryDsl;
use rocket::get;
use serde_json::json;

use crate::acl::guards::HasRole;
use crate::acl::models::Role;
use crate::acl::schema::roles;
use crate::roles::AdminRole;
use crate::{ApiResponse, DbConnection, ServerError};

#[get("/selectOptions", format = "application/json; charset=UTF-8")]
pub fn options(_perm: HasRole<AdminRole>, db: DbConnection) -> Result<ApiResponse, ServerError> {
    let options: Vec<serde_json::Value> = roles::table
        .load(db.as_ref())?
        .into_iter()
        .map(|item: Role| {
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

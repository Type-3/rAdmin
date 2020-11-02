use diesel::{PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use rocket::get;
use serde_json::json;

use crate::acl::guards::HasPermission;
use crate::acl::models::Permission;
use crate::acl::schema::permissions;
use crate::permissions::PermissionsList;
use crate::traits::Paginate;
use crate::{ApiResponse, DbConnection, ServerError};

#[get(
    "/tableData?<page>&<per_page>&<query>",
    format = "application/json; charset=UTF-8"
)]
pub fn data(
    _perm: HasPermission<PermissionsList>,
    db: DbConnection,
    per_page: Option<i64>,
    page: Option<i64>,
    query: Option<String>,
) -> Result<ApiResponse, ServerError> {
    let (items, total_pages, page, per_page) = if let Some(query) = query {
        permissions::table
            .filter(permissions::name.like(format!("%{}%", &query)))
            .or_filter(permissions::description.like(format!("%{}%", &query)))
            .or_filter(permissions::label.like(format!("%{}%", &query)))
            .paginate(page)
            .per_page(per_page)
            .load_and_count_pages::<Permission, PgConnection>(db.as_ref())?
    } else {
        permissions::table
            .select(permissions::all_columns)
            .paginate(page)
            .per_page(per_page)
            .load_and_count_pages::<Permission, PgConnection>(db.as_ref())?
    };
    let total = permissions::table.count().first::<i64>(db.as_ref())?;
    let items: Vec<serde_json::Value> = items.iter().map(|item| json!(item)).collect();
    let data = json!({
        "total": total,
        "total_pages": total_pages,
        "items": items,
        "page": page,
        "per_page": per_page
    });
    Ok(ApiResponse::ok().data(data))
}

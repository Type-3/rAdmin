use diesel::{PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use rocket::get;
use serde_json::json;

use crate::acl::guards::HasRole;
use crate::acl::models::Role;
use crate::acl::schema::roles;
use crate::roles::AdminRole;
use crate::traits::Paginate;
use crate::{ApiResponse, DbConnection, ServerError};

#[get(
    "/tableData?<page>&<per_page>&<query>",
    format = "application/json; charset=UTF-8"
)]
pub fn data(
    _perm: HasRole<AdminRole>,
    db: DbConnection,
    page: Option<i64>,
    per_page: Option<i64>,
    query: Option<String>,
) -> Result<ApiResponse, ServerError> {
    let (items, total_pages, page, per_page) = if let Some(query) = query {
        roles::table
            .filter(roles::name.like(format!("%{}%", &query)))
            .or_filter(roles::description.like(format!("%{}%", &query)))
            .or_filter(roles::label.like(format!("%{}%", &query)))
            .paginate(page)
            .per_page(per_page)
            .load_and_count_pages::<Role, PgConnection>(db.as_ref())?
    } else {
        roles::table
            .select(roles::all_columns)
            .paginate(page)
            .per_page(per_page)
            .load_and_count_pages::<Role, PgConnection>(db.as_ref())?
    };
    let total = roles::table.count().first::<i64>(db.as_ref())?;
    let items: Vec<serde_json::Value> = items.iter().map(|item| json!(item)).collect();
    let data = json!({
        "total_pages": total_pages,
        "total": total,
        "items": items,
        "page": page,
        "per_page": per_page
    });
    Ok(ApiResponse::ok().data(json!(data)))
}

use diesel::{PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use rocket::get;
use serde_json::json;

use crate::acl::guards::HasRole;
use crate::acl::models::Account;
use crate::acl::schema::accounts;
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
        accounts::table
            .filter(accounts::username.like(format!("%{}%", query)))
            .paginate(page)
            .per_page(per_page)
            .load_and_count_pages::<Account, PgConnection>(db.as_ref())?
    } else {
        accounts::table
            .select(accounts::all_columns)
            .paginate(page)
            .per_page(per_page)
            .load_and_count_pages::<Account, PgConnection>(db.as_ref())?
    };
    let total = accounts::table.count().first::<i64>(db.as_ref())?;
    let items: Vec<serde_json::Value> = items.iter().map(|item| json!(item)).collect();
    let data = json!({
        "total": total,
        "total_pages": total_pages,
        "page": page,
        "items": items,
        "per_page": per_page
    });
    Ok(ApiResponse::ok().data(data))
}

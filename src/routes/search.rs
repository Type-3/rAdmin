use rocket::{self, get, State};

use crate::acl::guards::AuthorizedAccount;
use crate::acl::traits::HasPermissions;
use crate::controllers::GlobalSearch;
use crate::controllers::SearchResult;
use crate::{ApiResponse, DbConnection, ServerError};

#[get("/search?<query>", format = "application/json")]
pub fn search(
    query: String,
    search: State<GlobalSearch>,
    account: AuthorizedAccount,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    let searcher = search.inner();
    let results: Vec<SearchResult> = searcher
        .query(&query)?
        .into_iter()
        .filter(|item| {
            matches!(
                account.0.has_permission_name(&item.permission, db.as_ref()),
                Ok(true)
            )
        })
        .collect();
    Ok(ApiResponse::ok().data(results))
}

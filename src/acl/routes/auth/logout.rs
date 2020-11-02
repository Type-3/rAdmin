use rocket::{self, post};

use crate::acl::guards::AuthorizedAccount;
use crate::acl::Auth;
use crate::{ApiResponse, DbConnection, ServerError};

#[post("/logout", format = "application/json; charset=UTF-8")]
pub fn logout(
    mut account: AuthorizedAccount,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    Auth::logout(&mut account.0, db.as_ref())?;
    Ok(ApiResponse::ok())
}

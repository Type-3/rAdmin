use rocket::{self, post};

use crate::acl::guards::AuthorizedAccount;
use crate::acl::Auth;
use crate::{ApiResponse, DbConnection, ServerError};
use rocket::response::{Flash, Redirect};
use rocket::http::{Cookies, Cookie};

#[post("/logout", format = "application/json; charset=UTF-8")]
pub fn api_logout(
    mut account: AuthorizedAccount,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    Auth::logout(&mut account.0, db.as_ref())?;
    Ok(ApiResponse::ok())
}

#[post("/logout")]
pub fn logout(mut account: AuthorizedAccount, mut cookies: Cookies, db: DbConnection) -> Result<Flash<Redirect>, ServerError> {
    Auth::logout(&mut account.0, db.as_ref())?;
    cookies.remove_private(Cookie::named("auth_token"));
    Ok(Flash::success(Redirect::to("/", ), "Bye Bye!"))
}
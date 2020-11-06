use diesel::prelude::*;
use rocket::{self, post, Config, State};
use rocket_contrib::json::Json;
use serde_json::json;

use crate::ServerError;
use crate::acl::guards::Unauthenticated;
use crate::acl::models::Account;
use crate::acl::schema::accounts::dsl::*;
use crate::acl::traits::{HasPermissions, HasRoles};
use crate::acl::Auth;
use crate::{ApiResponse, DbConnection};

#[post(
    "/login",
    data = "<account_in>",
    format = "application/json; charset=UTF-8"
)]
pub fn login(
    _auth: Unauthenticated,
    account_in: Json<crate::acl::requests::LoginRequest>,
    _app_config: State<Config>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    account_in.validate()?;

    let mut account = accounts
        .filter(username.eq(&account_in.username))
        .or_filter(email.eq(&account_in.username))
        .first::<Account>(&*db)
        .or_else(|_| {
            // Hash password here to prevent a timing attack.
            Auth::hash_nonsense(None).unwrap();
            Err(ServerError::Diesel(diesel::result::Error::NotFound))
        })?;

    if !Auth::perform_login(&mut account, &account_in.password, db.as_ref())? {
        return Ok(ApiResponse::unauthorized().message("Username or password incorrect."));
    }
    let response = json!({
        "id": account.id,
        "token": account.auth_token.as_ref().unwrap().clone(),
        "email": &account.email,
        "username": &account.username,
        "permissions": account.permission_names(db.as_ref())?,
        "roles": account.role_names(db.as_ref())?
    });

    Ok(ApiResponse::ok().data(response))
}

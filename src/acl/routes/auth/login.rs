use diesel::prelude::*;
use rocket::{self, post, Config, State};
use rocket_contrib::json::Json;
use serde_json::json;

use crate::acl::guards::Unauthenticated;
use crate::acl::models::Account;
use crate::acl::schema::accounts;
use crate::acl::Auth;
use crate::ServerError;
use crate::{ApiResponse, DbConnection};
use crate::acl::requests::LoginRequest;
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use rocket::http::Cookie;
use rocket::http::Cookies;

#[post(
    "/login",
    data = "<account_in>",
    format = "application/json; charset=UTF-8"
)]
pub fn api_login(
    _auth: Unauthenticated,
    account_in: Json<crate::acl::requests::LoginRequest>,
    _app_config: State<Config>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    match perform_login(account_in.into_inner(), db.as_ref())? {
        None => Ok(ApiResponse::unauthorized().message("Invalid Identifier/Password")),
        Some(account) => {
            Ok(ApiResponse::ok()
                .data(json!({
                    "id": account.id,
                    "token": account.auth_token.as_ref().unwrap().clone(),
                    "email": &account.email,
                    "avatar": &account.avatar,
                    "username": &account.username,
                    "roles": account.roles
                })))
        }
    }
}

#[post("/login", data = "<account_in>")]
pub fn form_login(_auth: Unauthenticated,
                  account_in: Form<crate::acl::requests::LoginRequest>,
                  _app_config: State<Config>,
                  mut cookies: Cookies,
                  db: DbConnection) -> Result<Flash<Redirect>, ServerError> {
    match perform_login(account_in.into_inner(), db.as_ref())? {
        None => Ok(Flash::error(Redirect::to("/auth/login"), "Invalid Username / Password")),
        Some(account) => {
            cookies.add_private(Cookie::new("auth_token", account.auth_token.unwrap()));
            Ok(Flash::success(Redirect::to("/"), ""))
        }
    }
}

fn perform_login(form: LoginRequest, db: &PgConnection) -> Result<Option<Account>, ServerError> {
    form.validate()?;
    let mut account = accounts::table
        .filter(accounts::username.eq(&form.identifier))
        .or_filter(accounts::email.eq(&form.identifier))
        .first::<Account>(&*db)
        .map_err(|_| {
            // Hash password here to prevent a timing attack.
            Auth::hash_nonsense(None).unwrap();
            ServerError::Diesel(diesel::result::Error::NotFound)
        })?;

    if !Auth::perform_login(&mut account, &form.password, db)? {
        Ok(None)
    } else {
        Ok(Some(account))
    }
}
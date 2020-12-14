use diesel::prelude::*;
use rocket::{self, post, Config, State};
use rocket_contrib::json;
use rocket_contrib::json::Json;

use crate::acl::guards::Unauthenticated;
use crate::acl::schema::accounts::dsl::*;
use crate::acl::Auth;
use crate::{ApiResponse, DbConnection, ServerError};
use crate::acl::models::Account;
use rocket::request::Form;
use crate::acl::requests::RegisterRequest;
use rocket::response::{Flash, Redirect};

#[post(
    "/register",
    data = "<account_in>",
    format = "application/json; charset=UTF-8"
)]
pub fn api_register(
    _auth: Unauthenticated,
    account_in: Json<RegisterRequest>,
    app_config: State<Config>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    match perform_register(account_in.into_inner(), app_config.inner(), db.as_ref())? {
        None => Ok(ApiResponse::unprocessable_entity(json!(null)).message("User / Email already exists")),
        Some(_) => Ok(ApiResponse::ok())
    }
}

#[post("/register", data = "<account_in>")]
pub fn form_register(
    _auth: Unauthenticated,
    account_in: Form<RegisterRequest>,
    app_config: State<Config>,
    db: DbConnection,
) -> Result<Flash<Redirect>, ServerError> {
    match perform_register(account_in.into_inner(), app_config.inner(), db.as_ref())? {
        None => Ok(Flash::error(Redirect::to("/auth/signup"), "User with that email already exists")),
        Some(_) => Ok(Flash::success(Redirect::to("/"), "Account Created"))
    }
}

fn perform_register(account_in: RegisterRequest, app_config: &Config, db: &PgConnection) -> Result<Option<Account>, ServerError> {
    account_in.validate()?;
    let account = accounts
        .filter(username.eq(&account_in.username))
        .or_filter(email.eq(&account_in.email))
        .count()
        .get_result::<i64>(&*db)?;

    if account > 0 {
        Ok(None)
    } else {
        Ok(Some(crate::acl::factories::AccountFactory::default()
            .email(&account_in.email)
            .username(&account_in.username)
            .roles(vec!["user".to_string()])
            .set_password_with_hash(
                Auth::get_password_hash(app_config),
                &account_in.password,
            )?
            .insert(db)))
    }
}

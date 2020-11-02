use diesel::prelude::*;
use rocket::{self, post, Config, State};
use rocket_contrib::json;
use rocket_contrib::json::Json;

use crate::acl::guards::Unauthenticated;
use crate::acl::schema::accounts::dsl::*;
use crate::acl::Auth;
use crate::{ApiResponse, DbConnection, ServerError};

#[post(
    "/register",
    data = "<account_in>",
    format = "application/json; charset=UTF-8"
)]
pub fn register(
    _auth: Unauthenticated,
    account_in: Json<crate::acl::requests::RegisterRequest>,
    app_config: State<Config>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    account_in.validate()?;

    let account = accounts
        .filter(username.eq(&account_in.username))
        .or_filter(email.eq(&account_in.email))
        .count()
        .get_result::<i64>(&*db)?;
    if account > 0 {
        Ok(ApiResponse::unprocessable_entity(
            json!({"message":"User with that Email/Username already exists"}),
        ))
    } else {
        crate::acl::factories::AccountFactory::default()
            .email(&account_in.email)
            .username(&account_in.username)
            .set_password_with_hash(
                Auth::get_password_hash(app_config.inner()),
                &account_in.password,
            )?
            .insert(db.as_ref());
        Ok(ApiResponse::ok().data(json!({"message": "Successfully created account"})))
    }
}

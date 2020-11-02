use diesel::{QueryDsl, RunQueryDsl, SaveChangesDsl};
use rocket::post;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use std::str::FromStr;

use crate::acl::guards::HasPermission;
use crate::acl::models::Account;
use crate::acl::schema::accounts;
use crate::permissions::AccountsModify;
use crate::traits::{Fillable, Validatable};
use crate::{ApiResponse, DbConnection, ServerError};

#[post(
    "/<account_id>",
    data = "<form>",
    format = "application/json; charset=UTF-8"
)]
pub fn update(
    _perm: HasPermission<AccountsModify>,
    account_id: Uuid,
    form: Json<crate::acl::requests::AccountRequest>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    let req = form.into_inner().validate()?;
    let uuid: ::uuid::Uuid = ::uuid::Uuid::from_str(&account_id.to_string()).unwrap();
    let mut account: Account = accounts::table.find(uuid).first(db.as_ref())?;
    req.fill(&mut account, db.as_ref())?;
    account.save_changes::<Account>(db.as_ref())?;
    Ok(ApiResponse::ok())
}

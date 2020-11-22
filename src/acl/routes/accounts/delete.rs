use diesel::QueryDsl;
use diesel::RunQueryDsl;
use rocket::{self, delete};
use rocket_contrib::uuid::Uuid;
use std::str::FromStr;

use crate::acl::guards::HasRole;
use crate::acl::models::Account;
use crate::acl::schema::accounts;
use crate::roles::AdminRole;
use crate::{ApiResponse, DbConnection, ServerError};

#[delete("/<account_id>", format = "application/json; charset=UTF-8")]
pub fn delete(
    account_id: Uuid,
    _perm: HasRole<AdminRole>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    // Convert from Rocket Uuid Type to uuid crate Uuid
    let uuid: ::uuid::Uuid = ::uuid::Uuid::from_str(&account_id.to_string()).unwrap();
    let account: Account = accounts::table.find(uuid).first::<Account>(db.as_ref())?;
    diesel::delete(&account).execute(db.as_ref())?;
    Ok(ApiResponse::ok())
}

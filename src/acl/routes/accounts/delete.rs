use crate::acl::models::Account;
use crate::crud::CrudDelete;
use crate::roles::AdminRole;
use std::str::FromStr;

use diesel::{RunQueryDsl, QueryDsl};
use rocket::response::{Flash, Redirect};
use crate::acl::schema::accounts;
use crate::{ServerError, DbConnection};
use crate::acl::guards::HasRole;

pub struct AccountsDelete;

impl CrudDelete<Account> for AccountsDelete {
    type Role = AdminRole;
}

#[rocket::get("/<account_id>")]
pub fn form_delete(
    account_id: String,
    _perm: HasRole<AdminRole>,
    db: DbConnection,
) -> Result<Flash<Redirect>, ServerError> {
    // Convert from Rocket Uuid Type to uuid crate Uuid
    let account = accounts::table.find(uuid::Uuid::from_str(&account_id)?);
    diesel::delete(account).execute(db.as_ref())?;
    Ok(Flash::success(Redirect::to("/admin/accounts"), format!("account {} removed", account_id)))
}

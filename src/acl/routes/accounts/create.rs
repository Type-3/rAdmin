use rocket::{post, Config, State};
use rocket_contrib::json::Json;

use crate::acl::forms::AccountCreateForm;
use crate::acl::guards::HasRole;
use crate::acl::Auth;
use crate::roles::AdminRole;
use crate::traits::{Submitable, Validatable};
use crate::{ApiResponse, DbConnection, ServerError};

#[post(
    "/create",
    data = "<account>",
    format = "application/json; charset=UTF-8"
)]
pub fn store(
    _perm: HasRole<AdminRole>,
    account: Json<crate::acl::requests::AccountRequest>,
    app_config: State<Config>,
    db: DbConnection,
) -> Result<ApiResponse, ServerError> {
    let req = account.into_inner().validate()?;
    let pw = Auth::get_password_hash(app_config.inner());
    let form = AccountCreateForm::from_req(pw, req);
    form.submit(db.as_ref())?;
    Ok(ApiResponse::ok())
}

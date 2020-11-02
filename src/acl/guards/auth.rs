use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::acl::models::Account;
use crate::DbConnection;

/// Verify the request came from an authorized user.
#[derive(Debug)]
pub struct AuthorizedAccount(pub Account);

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizedAccount {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthorizedAccount, ()> {
        let db = <DbConnection as FromRequest>::from_request(request)?;
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        };

        let token_header = keys[0];
        let token = token_header.replace("Bearer ", "");
        let account =
            Account::from_auth_token(&token, &*db).map_err(|_| Err((Status::Unauthorized, ())))?;
        Outcome::Success(AuthorizedAccount(account))
    }
}

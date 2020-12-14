use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::{Serialize, Deserialize};

use crate::acl::models::Account;
use crate::DbConnection;

/// Verify the request came from an authorized user.
#[derive(Debug, Serialize, Deserialize)]
pub struct PotentialAccount(pub Option<Account>);

impl<'a, 'r> FromRequest<'a, 'r> for PotentialAccount {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<PotentialAccount, ()> {
        let db = <DbConnection as FromRequest>::from_request(request)?;
        let token = if let Some(token) = request.cookies().get_private("auth_token") {
           token.value().to_string()
        } else {
            let keys: Vec<_> = request.headers().get("Authorization").collect();
            if keys.len() != 1 {
                return Outcome::Success(PotentialAccount(None));
            };

            let token_header = keys[0];
            token_header.replace("Bearer ", "")
        };
        let account =
            Account::from_auth_token(&token, &*db).map_err(|_| Err((Status::Unauthorized, ())))?;
        Outcome::Success(PotentialAccount(Some(account)))
    }
}
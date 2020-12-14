use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use std::marker::PhantomData;

use super::AuthorizedAccount;
use crate::acl::models::Account;
use crate::roles::RoleDef;

/// Generic request guard that will check that the authorized user
/// has a particular role.
pub struct HasRole<T: RoleDef> {
    _phantom: PhantomData<T>,
    pub account: Account
}

impl<'a, 'r, T: RoleDef> FromRequest<'a, 'r> for HasRole<T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<HasRole<T>, ()> {
        let account = request.guard::<AuthorizedAccount>()?.0;
        if account.roles.contains(&T::NAME.to_string()) {
            Outcome::Success(HasRole {
                _phantom: Default::default(),
                account
            })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

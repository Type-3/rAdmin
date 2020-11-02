use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use std::marker::PhantomData;

use super::AuthorizedAccount;
use crate::acl::traits::HasRoles;
use crate::roles::RoleDef;
use crate::DbConnection;

/// Generic request guard that will check that the authorized user
/// has a particular role.
pub struct HasRole<T: RoleDef> {
    _phantom: PhantomData<T>,
}

impl<'a, 'r, T: RoleDef> FromRequest<'a, 'r> for HasRole<T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<HasRole<T>, ()> {
        let account = request.guard::<AuthorizedAccount>()?.0;
        let db = request.guard::<DbConnection>()?.0;
        if account
            .has_role_name(T::NAME, &db)
            .map_err(|_| Err((Status::Unauthorized, ())))?
        {
            Outcome::Success(HasRole {
                _phantom: Default::default(),
            })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

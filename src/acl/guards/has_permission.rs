use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use std::marker::PhantomData;

use super::AuthorizedAccount;
use crate::acl::traits::HasPermissions;
use crate::permissions::PermissionDef;
use crate::DbConnection;

/// Generic request guard that will check that the authorized user
/// has a particular permission.
pub struct HasPermission<T: PermissionDef> {
    _phantom: PhantomData<T>,
}

impl<'a, 'r, T: PermissionDef> FromRequest<'a, 'r> for HasPermission<T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<HasPermission<T>, ()> {
        let account = request.guard::<AuthorizedAccount>()?.0;
        let db = request.guard::<DbConnection>()?.0;
        if account
            .has_permission_name(T::NAME, &db)
            .map_err(|_| Err((Status::Unauthorized, ())))?
        {
            Outcome::Success(HasPermission {
                _phantom: Default::default(),
            })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use std::marker::PhantomData;

use super::AuthorizedAccount;
use crate::acl::models::Role;
use crate::acl::schema::roles;
use crate::roles::RoleDef;
use crate::DbConnection;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

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
        let role: Role = roles::table
            .filter(roles::name.eq(T::NAME))
            .first(&db)
            .map_err(|_| Err((Status::Unauthorized, ())))?;

        if account.roles.contains(&role.id) {
            Outcome::Success(HasRole {
                _phantom: Default::default(),
            })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

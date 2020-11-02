use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

/// Verify the Request Came from an unauthenticated user.
#[derive(Debug)]
pub struct Unauthenticated;

impl<'a, 'r> FromRequest<'a, 'r> for Unauthenticated {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Unauthenticated, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.is_empty() {
            Outcome::Success(Unauthenticated)
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

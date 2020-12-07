mod result;
pub use self::result::SelectResult;
use crate::roles::RoleDef;
use rocket::http::Method;
use diesel::PgConnection;
use crate::{ServerError, DbConnection, ApiResponse};
use rocket::{Request, Data};
use crate::acl::guards::HasRole;
use rocket::request::FromRequest;
use rocket::handler::Outcome;

pub trait ApiSelect: 'static {
    type Role: RoleDef;
    const METHOD: Method = Method::Get;
    const URL: &'static str = "/selectOptions";
    fn query(db: &PgConnection) -> Result<Vec<SelectResult>, ServerError>;

    fn check_permission<'r>(req: &'r Request) -> Result<(), Option<Outcome<'r>>> {
        match <HasRole<Self::Role> as FromRequest>::from_request(req) {
            rocket::Outcome::Success(_) => Ok(()),
            rocket::Outcome::Failure(e) => Err(Some(Outcome::Failure(e.0))),
            rocket::Outcome::Forward(_) => Err(None)
        }
    }

    fn get_database<'r>(req: &'r Request) -> Result<DbConnection, Option<Outcome<'r>>> {
        match DbConnection::from_request(req) {
            rocket::Outcome::Success(db) => Ok(db),
            rocket::Outcome::Failure(err) => Err(Some(Outcome::Failure(err.0))),
            rocket::Outcome::Forward(_) => Err(None)
        }
    }

    fn handler<'r>(req: &'r Request, data: Data) -> Outcome<'r> {
        match Self::check_permission(req) {
            Ok(_) => {},
            Err(None) => return Outcome::Forward(data),
            Err(Some(err)) => return err
        }

        let db = match Self::get_database(req) {
            Ok(db) => db,
            Err(None) => return Outcome::Forward(data),
            Err(Some(err)) => return err
        };

        match Self::query(db.as_ref()) {
            Ok(res) => Outcome::from(req, ApiResponse::ok().data(res)),
            Err(err) => Outcome::from(req, err)
        }
    }

    fn route() -> rocket::Route {
        rocket::Route::new(Self::METHOD, Self::URL, Self::handler)
    }
}
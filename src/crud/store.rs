use crate::acl::guards::HasRole;
use crate::roles::RoleDef;
use crate::traits::{Submitable, Validatable};
use crate::{ApiResponse, DbConnection, ServerError};
use rocket::data::{FromData, Transform};
use rocket::handler::Outcome;
use rocket::http::Method;
use rocket::request::FromRequest;
use rocket::Data;
use rocket_contrib::json::Json;
use serde::de::DeserializeOwned;

pub trait CrudStore: 'static {
    type Role: RoleDef;
    type Form: Submitable;
    type Request: Validatable + Into<Self::Form> + DeserializeOwned;

    const METHOD: Method = Method::Post;
    const URL: &'static str = "/store";

    fn check_permission<'r>(req: &'r rocket::Request) -> Result<(), Option<Outcome<'r>>> {
        match HasRole::<Self::Role>::from_request(req) {
            rocket::Outcome::Success(_) => Ok(()),
            rocket::Outcome::Failure(e) => Err(Some(Outcome::Failure(e.0))),
            rocket::Outcome::Forward(_) => Err(None),
        }
    }

    fn get_database<'r>(req: &'r rocket::Request) -> Result<DbConnection, Option<Outcome<'r>>> {
        match DbConnection::from_request(req) {
            rocket::Outcome::Success(db) => Ok(db),
            rocket::Outcome::Failure(err) => Err(Some(Outcome::Failure(err.0))),
            rocket::Outcome::Forward(_) => Err(None),
        }
    }

    fn get_request<'r>(req: &'r rocket::Request, data: Data) -> Result<Self::Request, Outcome<'r>> {
        let __transform = Json::<Self::Request>::transform(req, data);
        let __outcome = match __transform {
            Transform::Owned(rocket::Outcome::Success(__v)) => {
                Transform::Owned(rocket::Outcome::Success(__v))
            }
            Transform::Borrowed(rocket::Outcome::Success(ref __v)) => {
                Transform::Borrowed(rocket::Outcome::Success(&__v[..]))
            }
            Transform::Borrowed(__o) => Transform::Borrowed(__o.map(|_| unreachable!())),
            Transform::Owned(__o) => Transform::Owned(__o),
        };
        match <Json<Self::Request> as FromData>::from_data(req, __outcome) {
            rocket::Outcome::Success(__d) => Ok(__d.into_inner()),
            rocket::Outcome::Forward(__d) => Err(rocket::Outcome::Forward(__d)),
            rocket::Outcome::Failure(__c) => Err(rocket::Outcome::Failure(__c.0)),
        }
    }

    fn handler<'r>(req: &'r rocket::Request, data: Data) -> Outcome<'r> {
        match Self::check_permission(req) {
            Ok(_) => {}
            Err(None) => return Outcome::Forward(data),
            Err(Some(err)) => return err,
        }

        let db = match Self::get_database(req) {
            Ok(db) => db,
            Err(None) => return Outcome::Forward(data),
            Err(Some(err)) => return err,
        };

        match Self::get_request(req, data) {
            Err(output) => output,
            Ok(form_req) => match form_req.validate() {
                Err(err) => Outcome::from(req, ServerError::Validator(err)),
                Ok(form_req) => {
                    let form: Self::Form = form_req.into();
                    match form.submit(db.as_ref()) {
                        Err(err) => Outcome::from(req, err),
                        Ok(_) => Outcome::from(req, ApiResponse::ok()),
                    }
                }
            },
        }
    }

    fn route() -> rocket::Route {
        rocket::Route::new(Self::METHOD, Self::URL, Self::handler)
    }
}

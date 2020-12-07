use rocket::handler::Outcome;

use rocket::http::Method;
use crate::roles::RoleDef;
use rocket::{Request, Data};
use crate::acl::guards::HasRole;
use rocket::request::FromRequest;
use crate::{DbConnection, ApiResponse, ServerError};
use diesel::PgConnection;

mod response;
pub use self::response::TableResponse;

pub trait ApiTable: 'static {
    type Role: RoleDef;
    type Model: serde::Serialize;
    const URL: &'static str = "/tableData?<query>&<per_page>&<page>";
    const METHOD: Method = Method::Get;

    fn query(conn: &PgConnection, query: Option<String>, page: Option<i64>, per_page: Option<i64>) -> Result<(Vec<Self::Model>, i64, i64, i64, i64), ServerError>;

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

    fn get_query_page<'r>(req: &'r Request) -> Result<Option<i64>, Option<Outcome<'r>>> {
        match req.get_query_value("page") {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(err)) => Err(Some(Outcome::from(req, format!("{:?}", err)))),
            None => Ok(None)
        }
    }

    fn get_query_per_page<'r>(req: &'r Request) -> Result<Option<i64>, Option<Outcome<'r>>> {
        match req.get_query_value("per_page") {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(err)) => Err(Some(Outcome::from(req, format!("Invalid page: {:?}", err)))),
            None => Ok(None)
        }
    }

    fn get_query_query<'r>(req: &'r Request) -> Result<Option<String>, Option<Outcome<'r>>> {
        match req.get_query_value("query") {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(err)) => Err(Some(Outcome::from(req, format!("Invalid Query: {:?}", err)))),
            None => Ok(None)
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

        let page = match Self::get_query_page(req) {
            Ok(page) => page,
            Err(Some(err)) => return err,
            Err(None) => return Outcome::from(req, ApiResponse::internal_server_error())
        };

        let per_page = match Self::get_query_per_page(req) {
            Ok(page) => page,
            Err(Some(err)) => return err,
            Err(None) => return Outcome::from(req, ApiResponse::internal_server_error())
        };

        let query = match Self::get_query_query(req) {
            Ok(page) => page,
            Err(Some(err)) => return err,
            Err(None) => return Outcome::from(req, ApiResponse::internal_server_error())
        };
        match Self::query(db.as_ref(), query, page, per_page) {
            Ok((items, total, total_pages, page, per_page)) => {
                let res = TableResponse {
                    total_pages,
                    total,
                    page,
                    per_page,
                    items: items.iter().map(|item|serde_json::to_value(item).unwrap()).collect(),
                };
                Outcome::from(req, ApiResponse::ok().data(res))
            },
            Err(err) => Outcome::from(req, err)
        }
    }

    fn route() -> rocket::Route {
        rocket::Route::new(Self::METHOD, Self::URL, Self::handler)
    }
}
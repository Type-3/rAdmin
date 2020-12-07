use crate::acl::guards::HasRole;
use crate::roles::RoleDef;
use crate::{ApiResponse, DbConnection};
use diesel::associations::HasTable;
use diesel::helper_types::Find;
use diesel::pg::Pg;
use diesel::query_builder::{IntoUpdateTarget, QueryFragment, QueryId};
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::{PgConnection, QueryDsl, QuerySource, RunQueryDsl};
use rocket::handler::Outcome;
use rocket::http::Method;
use rocket::request::{FromParam, FromRequest};
use rocket::{Data, Request};
use std::str::FromStr;

pub trait CrudDelete<Model: 'static>
where
    Self: 'static,
    Model: HasTable,
    <Model as HasTable>::Table: FindDsl<uuid::Uuid>,
    <<Model as HasTable>::Table as FindDsl<uuid::Uuid>>::Output: IntoUpdateTarget,
    Find<<Model as HasTable>::Table, uuid::Uuid>: RunQueryDsl<PgConnection>,
    <<<Model as HasTable>::Table as FindDsl<uuid::Uuid>>::Output as HasTable>::Table: QueryId,
    <<<<Model as HasTable>::Table as FindDsl<uuid::Uuid>>::Output as HasTable>::Table as QuerySource>::FromClause: QueryFragment<Pg>,
    <<<Model as HasTable>::Table as FindDsl<uuid::Uuid>>::Output as IntoUpdateTarget>::WhereClause: QueryFragment<Pg> + QueryId
{
    type Role: RoleDef;
    const METHOD: Method = Method::Delete;
    const URL: &'static str = "/<id>";

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

    fn get_id_param<'r>(req: &'r Request) -> Result<rocket_contrib::uuid::Uuid, Option<Outcome<'r>>> {
        match req.raw_segment_str(0) {
            None => Err(None),
            Some(w) => match rocket_contrib::uuid::Uuid::from_param(w) {
                Ok(uuid) => Ok(uuid),
                Err(_) => Err(None)
            }
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

        let id_param = match Self::get_id_param(req) {
            Ok(uuid) => {
                let id = uuid.into_inner();
                ::uuid::Uuid::from_str(&id.to_string()).unwrap()
            },
            Err(None) => return Outcome::Forward(data),
            Err(Some(outcome)) => return outcome
        };
        let model = QueryDsl::find(Model::table(), id_param);
        match diesel::delete(model).execute(db.as_ref()) {
                Ok(_) => Outcome::from(req, ApiResponse::ok()),
                Err(_) => Outcome::from(req, ApiResponse::internal_server_error())
        }
    }

    fn route() -> rocket::Route {
        rocket::Route::new(Self::METHOD, Self::URL, Self::handler)
    }
}

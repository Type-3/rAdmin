use derive_more::From;
use rocket::config::ConfigError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::json;
use serde_json::Error as JsonError;
use std::env::VarError;
use std::str::Utf8Error;
use validator::ValidationErrors;

use std::io::Error as IoError;

use super::ApiResponse;

#[derive(Debug, From)]
pub enum ServerError {
    HashError,
    Uuid(uuid::Error),
    Json(JsonError),
    Var(VarError),
    Io(IoError),
    Utf8Str(Utf8Error),
    Config(ConfigError),
    Launch(rocket::error::LaunchError),
    Migrations(diesel::migration::RunMigrationsError),
    Connection(diesel::ConnectionError),
    Diesel(diesel::result::Error),
    Validator(ValidationErrors),
}

impl<'r> Responder<'r> for ServerError {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        if let ServerError::Validator(errors) = &self {
            ApiResponse::unprocessable_entity(json!(errors)).respond_to(req)
        } else {
            ApiResponse::internal_server_error().respond_to(req)
        }
    }
}

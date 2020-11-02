use rocket::catch;
use rocket::response::{Flash, Redirect};
use rocket::Request;

use crate::ApiResponse;

#[catch(400)]
pub fn bad_request_handler() -> ApiResponse {
    ApiResponse::bad_request()
}

#[catch(401)]
pub fn unauthorized_handler() -> ApiResponse {
    ApiResponse::unauthorized()
}

#[catch(403)]
pub fn forbidden_handler() -> ApiResponse {
    ApiResponse::forbidden()
}

#[catch(404)]
pub fn not_found_handler(req: &Request) -> Flash<Redirect> {
    Flash::new(Redirect::to("/"), "REQUEST_URI", req.uri().path())
}

#[catch(500)]
pub fn internal_server_error_handler() -> ApiResponse {
    ApiResponse::internal_server_error()
}

#[catch(503)]
pub fn service_unavailable_handler() -> ApiResponse {
    ApiResponse::service_unavailable()
}

pub fn api_errors() -> Vec<rocket::Catcher> {
    rocket::catchers![
        forbidden_handler,
        not_found_handler,
        bad_request_handler,
        unauthorized_handler,
        service_unavailable_handler,
        internal_server_error_handler,
    ]
}

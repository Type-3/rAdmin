use diesel::result::Error as DieselError;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

use std::convert::From;
use std::io::Cursor;

#[derive(Debug)]
pub struct ApiResponse {
    data: JsonValue,
    status: Status,
}

impl ApiResponse {
    /// Set the data of the `Response` to `data`.
    pub fn data<S: serde::Serialize>(mut self, data: S) -> ApiResponse {
        self.data = serde_json::from_str(&serde_json::to_string(&data).unwrap()).unwrap();
        self
    }

    /// Convenience method to set `self.data` to `{"message": message}`.
    pub fn message(mut self, message: &str) -> ApiResponse {
        self.data = json!({ "message": message });
        self
    }

    pub fn ok() -> ApiResponse {
        ApiResponse {
            data: json!(null),
            status: Status::Ok,
        }
    }

    pub fn created() -> ApiResponse {
        ApiResponse {
            data: json!(null),
            status: Status::Created,
        }
    }

    pub fn accepted() -> ApiResponse {
        ApiResponse {
            data: json!(null),
            status: Status::Accepted,
        }
    }

    pub fn no_content() -> ApiResponse {
        ApiResponse {
            data: json!(null),
            status: Status::NoContent,
        }
    }

    pub fn bad_request() -> ApiResponse {
        ApiResponse {
            data: json!({"message": "Bad Request"}),
            status: Status::BadRequest,
        }
    }

    pub fn unauthorized() -> ApiResponse {
        ApiResponse {
            data: json!({"message": "Unauthorized"}),
            status: Status::Unauthorized,
        }
    }

    pub fn forbidden() -> ApiResponse {
        ApiResponse {
            data: json!({"message": "Forbidden"}),
            status: Status::Forbidden,
        }
    }

    pub fn not_found() -> ApiResponse {
        ApiResponse {
            data: json!({"message": "Not Found"}),
            status: Status::NotFound,
        }
    }

    pub fn method_not_allowed() -> ApiResponse {
        ApiResponse {
            data: json!({"message": "Method Not Allowed"}),
            status: Status::MethodNotAllowed,
        }
    }

    pub fn conflict() -> ApiResponse {
        ApiResponse {
            data: json!({"message": "Conflict"}),
            status: Status::Conflict,
        }
    }

    pub fn unprocessable_entity(errors: JsonValue) -> ApiResponse {
        ApiResponse {
            data: json!({ "message": errors }),
            status: Status::UnprocessableEntity,
        }
    }

    pub fn internal_server_error() -> ApiResponse {
        ApiResponse {
            data: json!({"message": "Internal Server Error"}),
            status: Status::InternalServerError,
        }
    }

    pub fn service_unavailable() -> ApiResponse {
        ApiResponse {
            data: json!({"message": "Service Unavailable"}),
            status: Status::ServiceUnavailable,
        }
    }
}

impl From<super::ServerError> for ApiResponse {
    fn from(err: super::ServerError) -> Self {
        ApiResponse::internal_server_error().data(json!(format!("{:?}", err)))
    }
}

impl From<DieselError> for ApiResponse {
    fn from(err: DieselError) -> Self {
        ApiResponse::internal_server_error().data(json!(format!("{:?}", err)))
    }
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let body = self.data;

        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

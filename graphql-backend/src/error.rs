use actix::MailboxError;
// use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use jwt::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use libreauth::pass::ErrorCode as PassErrorCode;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::convert::From;
use validator::ValidationErrors;

#[derive(Serialize, Deserialize, Debug, async_graphql::SimpleObject)]
struct ValidationError {
    message: String,
    key: String,
}

#[derive(Fail, Debug)]
pub enum Error {
    // 401
    #[fail(display = "{}", _0)]
    Unauthorized(String),

    // 403
    #[fail(display = "Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[fail(display = "Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    #[fail(display = "Validation Errors")]
    ValidationErrors(Vec<ValidationError>),

    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
// impl ResponseError for Error {
//     fn error_response(&self) -> HttpResponse {
//         match *self {
//             Error::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
//             Error::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
//             Error::NotFound(ref message) => HttpResponse::NotFound().json(message),
//             Error::UnprocessableEntity(ref message) => {
//                 HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
//             }
//             Error::InternalServerError => {
//                 HttpResponse::InternalServerError().json("Internal Server Error")
//             }
//         }
//     }
// }

impl From<MailboxError> for Error {
    fn from(_error: MailboxError) -> Self {
        Error::InternalServerError
    }
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        match error.kind() {
            JwtErrorKind::InvalidToken => Error::Unauthorized("Token is invalid".to_string()),
            JwtErrorKind::InvalidIssuer => Error::Unauthorized("Issuer is invalid".to_string()),
            _ => Error::Unauthorized("An issue was found with the token provided".to_string()),
        }
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessableEntity(json!({ "error": message }));
                }
                Error::InternalServerError
            }
            DieselError::NotFound => {
                Error::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => Error::InternalServerError,
        }
    }
}

impl From<PoolError> for Error {
    fn from(_error: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<PassErrorCode> for Error {
    fn from(_error: PassErrorCode) -> Self {
        Error::InternalServerError
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut errs: Vec<ValidationError> = Vec::new();

        for (field, errors) in errors.field_errors().iter() {
            let error_messages: Vec<String> = errors
                .iter()
                .filter_map(|error| error.message.clone().map(|message| message.into_owned()))
                .collect();
            errs.push(ValidationError {
                message: error_messages.join(", "),
                key: field.to_string(),
            });
        }

        Error::ValidationErrors(errs)
    }
}

// converts validation errors to Error
pub fn validation_errors_to_error(errors: ValidationErrors) -> Error {
    let mut errs: Vec<ValidationError> = Vec::new();

    for (field, errors) in errors.field_errors().iter() {
        let error_messages: Vec<String> = errors
            .iter()
            .filter_map(|error| error.message.clone().map(|message| message.into_owned()))
            .collect();
        errs.push(ValidationError {
            message: error_messages.join(", "),
            key: field.to_string(),
        });
    }

    Error::ValidationErrors(errs)
}

impl async_graphql::ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::FieldError {
        self.extend_with(|err, e| match err {
            Error::ValidationErrors(errs) => {
                let json_errors = async_graphql::Value::from_json(json!(errs)).unwrap();
                e.set("errors", json_errors);
            }
            _ => e.set("code", "NOT_FOUND"),
        })
    }
}

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error};
use std::convert::From;
use uuid::ParseError;
use actix::MailboxError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try later"),
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json("Unauthorized")
            }
        }
    }
}

impl From<MailboxError> for ServiceError {
    fn from(_: MailboxError) -> ServiceError {
        ServiceError::InternalServerError
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<Error> for ServiceError {
    fn from(error: Error) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            Error::DatabaseError(kind, info) => {
                // if let DatabaseErrorKind::UniqueViolation = kind {
                    let message =
                        info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                // }
                // ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

use super::auth::ErrorCodes;

#[derive(Error, Debug)]
pub enum HttpErrorCodes {
    // #[error("2001")] : "2001" is for bad request
    #[error("2001")]
    BadRequest,

    // #[error("2002")] : "2002" is for not found
    #[error("2002")]
    NotFound,

    // #[error("2003")] : "2003" is for unknown
    #[error("2003")]
    Unknown,

    // #[error("2004")] : "2004" is for unauthorized
    #[error("2004")]
    Unauthorized,
}

impl HttpErrorCodes {
    pub fn descriptor(&self) -> String {
        match self {
            Self::BadRequest => String::from("Unable to process request"),
            Self::Unknown => String::from("Internal server error"),
            Self::NotFound => String::from("Resource was not found"),
            Self::Unauthorized => String::from("Not authorized"),
        }
    }
}

// Error response
#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ResponseError for HttpErrorCodes {
    // Convert error to http response
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    // Convert error to http response
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.descriptor(),
            error: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<diesel::result::Error> for HttpErrorCodes {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => HttpErrorCodes::NotFound,
            _ => HttpErrorCodes::Unknown,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for HttpErrorCodes {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e {
            _ => HttpErrorCodes::Unknown,
        }
    }
}

impl From<ErrorCodes> for HttpErrorCodes {
    fn from(e: ErrorCodes) -> Self {
        match e {
            _ => Self::Unauthorized,
        }
    }
}

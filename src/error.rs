use crate::response::Response;

use axum::{response::IntoResponse, Json};

#[derive(Debug)]
pub enum ErrorType {
    DbError,
    ServerError,
    DeserializerError,

    PostNotFound,
    CategoryNotFound,
    TagNotFound,

    Unauthorized,

    RequestError,

    ConfigError,
}

#[derive(Debug)]
pub struct Error {
    pub cause: String,
    pub message: String,
    pub err_type: ErrorType,
}

impl Error {
    pub fn new(cause: &str, msg: &str, err_type: ErrorType) -> Self {
        Self {
            cause: cause.to_string(),
            message: String::from(msg),
            err_type,
        }
    }

    fn code(&self) -> i32 {
        match self.err_type {
            ErrorType::ConfigError => 0,

            ErrorType::RequestError => 400,

            ErrorType::Unauthorized => 401,

            ErrorType::PostNotFound => 404,
            ErrorType::CategoryNotFound => 404,
            ErrorType::TagNotFound => 404,

            ErrorType::ServerError => 500,
            ErrorType::DbError => 500,
            ErrorType::DeserializerError => 500,
        }
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!("{}:{}", self.cause, self.message)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let code = (&self).code();
        let msg = format!("{}:{}", self.cause, self.message);

        let res: Response<()> = Response::err(code, &msg);
        Json(res).into_response()
    }
}

impl From<axum::Error> for Error {
    fn from(e: axum::Error) -> Self {
        Error {
            cause: "axum".to_string(),
            message: e.to_string(),
            err_type: ErrorType::ServerError,
        }
    }
}

impl From<sea_orm::error::DbErr> for Error {
    fn from(e: sea_orm::error::DbErr) -> Self {
        Error {
            cause: "database".to_string(),
            message: e.to_string(),
            err_type: ErrorType::DbError,
        }
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Error {
            cause: "config".to_string(),
            message: e.to_string(),
            err_type: ErrorType::ConfigError,
        }
    }
}

impl From<axum::extract::rejection::JsonRejection> for Error {
    fn from(e: axum::extract::rejection::JsonRejection) -> Self {
        Error {
            cause: "request".to_string(),
            message: e.to_string(),
            err_type: ErrorType::RequestError,
        }
    }
}

use std::fmt::{Formatter};
use actix_web::HttpResponse;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Error {
    ErrorCode: i16,
    ErrorMessage: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorCode : {}, ErrorMessage : {}", self.ErrorCode, self.ErrorMessage)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub ErrorCode: i16,
    pub ErrorMessage: String,
}

impl Into<Error> for RepositoryError {
    fn into(self) -> Error {
        Error {
            ErrorCode: self.ErrorCode,
            ErrorMessage: self.ErrorMessage,
        }
    }
}

#[derive(Debug)]
pub struct ApiError(Error);


impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        ApiError(value)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(&self.0)
    }
}
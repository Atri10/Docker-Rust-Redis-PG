use crate::domain::error::RepositoryError;
pub use actix_threadpool::{BlockingError};
use diesel::r2d2;

pub type AsyncPoolError<T> = BlockingError<T>;


#[derive(Debug)]
pub struct DieselRepositoryError(RepositoryError);

impl DieselRepositoryError {
    pub fn into_inner(self) -> RepositoryError {
        self.0
    }
}

impl From<(r2d2::Error, i16)> for DieselRepositoryError {
    fn from((error, code): (r2d2::Error, i16)) -> Self {
        DieselRepositoryError(
            RepositoryError {
                ErrorCode: code,
                ErrorMessage: error.to_string(),
            }
        )
    }
}

impl From<(diesel::result::Error, i16)> for DieselRepositoryError {
    fn from((error, code): (diesel::result::Error, i16)) -> Self {
        DieselRepositoryError(
            RepositoryError {
                ErrorCode: code,
                ErrorMessage: error.to_string(),
            }
        )
    }
}


impl<T: std::fmt::Debug> From<(AsyncPoolError<T>, i16)> for DieselRepositoryError {
    fn from((error, code): (AsyncPoolError<T>, i16)) -> Self {
        DieselRepositoryError(
            RepositoryError {
                ErrorCode: code,
                ErrorMessage: error.to_string(),
            }
        )
    }
}







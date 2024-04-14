use crate::domain::error::RepositoryError;

pub type RepositoryResult<T> = Result<T, RepositoryError>;
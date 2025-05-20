use crate::domain::error::RepoError;

use super::pool::AsyncPoolError;

impl From<diesel::r2d2::Error> for RepoError{
    fn from(error: diesel::r2d2::Error) -> Self {
            RepoError{
                message: error.to_string()
            }
    }
}
impl From<r2d2::Error> for RepoError{
    fn from(error: r2d2::Error) -> Self {
            RepoError{
                message: error.to_string()
            }
    }
}

impl From<diesel::result::Error> for RepoError {
    fn from(value: diesel::result::Error) -> Self {
        RepoError{
            message: value.to_string(),
        }
    }
}

impl From<AsyncPoolError> for RepoError {
    fn from(value: AsyncPoolError) -> Self {
        RepoError{
            message: value.to_string(),
        }
    }
    
}

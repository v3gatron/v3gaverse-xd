use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("400 Bad Request")]
    BadRequest,
    #[error("401 Unauthorized")]
    Unauthorized,
    #[error("403 Forbidden")]
    Forbidden,
    #[error("404 Not Found")]
    NotFound,
    #[error("500 Internal Server Error")]
    InternalServerError,
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),
}

// NOTE: watch out for this. 'thiserror/anyhow'
#[cfg(feature = "thiserror")]
impl From<Error> for anyhow::Error {
    fn from(error: Error) -> Self {
        anyhow::Error::new(error.to_string())
    }
}

use tokio_pg_mapper::Error as PGMError;
use deadpool_postgres::PoolError;
use tokio_postgres::error::Error as PGError;
use std::error::Error;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum CustomError {
  NotFound,
  PGError(PGError),
  PGMError(PGMError),
  PoolError(PoolError),
}

// Implement the Error trait for CustomError
impl Error for CustomError {}

impl ResponseError for CustomError {
  fn error_response(&self) -> HttpResponse {
    match *self {
      CustomError.NotFound => HttpResponse::NotFound().finish(),
      CustomError::PoolError(ref err) => {
        HttpResponse::InternalServerError().body(err.to_string())
      },
      _ => HttpResponse::InternalServerError().finish(),
    }
  }
}

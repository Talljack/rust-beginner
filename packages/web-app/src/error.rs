use actix_web::{
  error,
  HttpResponse,
  http::header::ContentType,
  http::StatusCode
};

use derive_more::{Display, Error};

#[allow(dead_code)]
#[derive(Debug, Display, Error)]
pub enum MyError {
  #[display(fmt = "internal server error")]
  InternalServerError,
  #[display(fmt = "bad request")]
  BadRequest,
  #[display(fmt = "timeout")]
  TimedOut,
  #[display(fmt = "pool error")]
  PoolError
}

impl error::ResponseError for MyError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code())
      .insert_header(ContentType::html())
      .body(self.to_string())
  }
  fn status_code(&self) -> StatusCode {
    match *self {
      MyError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      MyError::BadRequest => StatusCode::BAD_REQUEST,
      MyError::TimedOut => StatusCode::GATEWAY_TIMEOUT,
      MyError::PoolError => StatusCode::SERVICE_UNAVAILABLE,
    }
  }
}

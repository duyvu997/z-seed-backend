use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::constance;

#[derive(Serialize)]
pub struct Data<T> {
  pub data: T,
  pub message: String
}

pub enum ApiResponse<T> {
  Ok(Data<T>),
  Created(Data<T>)
}

impl<T> IntoResponse for ApiResponse<T> where T: Serialize {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::Ok(data) => (StatusCode::OK, Json(data)).into_response(),
      Self::Created(data) => (StatusCode::CREATED, Json(data)).into_response()
    }
  }
}

pub enum ApiError {
  BadRequest(String),
  NotFound(String),
  InternalServiceError(String)
}

impl IntoResponse for ApiError {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::BadRequest(message) => (
        StatusCode::BAD_REQUEST,
        Json(if message.is_empty() {constance::BAD_REQUEST.to_string()} else {message})).into_response(),
      Self::NotFound(message) => (
        StatusCode::NOT_FOUND,
        Json(if message.is_empty() {constance::NOT_FOUND.to_string()} else {message})).into_response(),
      Self::InternalServiceError(message) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(if message.is_empty() {constance::INTERNAL_SERVER_ERROR.to_string()} else {message})).into_response()
    }
  }
}

use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{common::{ApiError, ApiResponse, Data}, constants, dtos::user_dto::CreateRequest, entities::user::User};

pub async fn get_user(State(data): State<Arc<Pool<Postgres>>>, Path(id): Path<String>) -> Result<ApiResponse<User>, ApiError> {
  // Validate UUID format
  if let Err(_) = Uuid::parse_str(&id) {
      return Err(ApiError::BadRequest("Invalid UUID format".to_string()));
  }

  let result = sqlx::query_as::<_,User>("SELECT * FROM users WHERE id = $1")
    .bind(&id)
    .fetch_one(&*data)
    .await;

  match result {
    Ok(data) => Ok(ApiResponse::Ok(Data {data, message: constants::SUCCESS.to_string()})),
    Err(sqlx::Error::RowNotFound) => Err(ApiError::NotFound("User not found".to_string())),
    Err(err) => Err(ApiError::InternalServiceError(format!("Database error: {}", err)))
  }
}

pub async fn create(State(data): State<Arc<Pool<Postgres>>>, Json(user): Json<CreateRequest>) -> Result<ApiResponse<User>, ApiError> {
  let id = Uuid::new_v4().to_string();
  let response = sqlx::query("INSERT INTO users (id, username) VALUES ($1, $2)")
    .bind(&id)
    .bind(&user.username)
    .execute(&*data)
    .await;

  if let Err(err) = response {
    return Err(ApiError::InternalServiceError(err.to_string()));
  }

  let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
    .bind(&id)
    .fetch_one(&*data)
    .await;

  match result {
    Ok(data) => Ok(ApiResponse::Created(Data {data, message: constants::CREATED.to_string()})),
    Err(err) => Err(ApiError::BadRequest(err.to_string()))
  }
}

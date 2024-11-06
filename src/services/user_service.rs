use axum::{
  extract::{Path, State},
  Json,
};
use diesel::{
  r2d2::{ConnectionManager, Pool},
  PgConnection, QueryDsl, RunQueryDsl,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
  common::{ApiError, ApiResponse, Data},
  constants,
  dtos::user_dto::{CreateRequest, UserDto},
  entities::user::User,
  schema::users,
};

#[utoipa::path(
  get,
  path = "/api/users/{id}",
  params(
    ("id" = &str, Path, description = "User ID")
  ),
  responses(
    (status = 200, description = "Get user by id", body = Data<UserDto>)
  )
)]
pub async fn get_by_id(
  State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
  Path(id): Path<String>,
) -> Result<ApiResponse<User>, ApiError> {
  // Validate UUID format
  if Uuid::parse_str(&id).is_err() {
    return Err(ApiError::BadRequest("Invalid UUID format".to_string()));
  }

  let mut conn = pool.get().unwrap();
  let result = users::table.find(id).first::<User>(&mut conn);

  match result {
    Ok(data) => Ok(ApiResponse::Ok(Data { data, message: constants::SUCCESS.to_string() })),
    Err(err) => Err(ApiError::InternalServiceError(format!("Database error: {}", err))),
  }
}

#[utoipa::path(
  post,
  path = "/api/users",
  request_body = CreateRequest,
  responses(
    (status = 200, description = "Create user", body = Data<UserDto>)
  )
)]
pub async fn create(
  State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
  Json(user): Json<CreateRequest>,
) -> Result<ApiResponse<User>, ApiError> {
  let id = Uuid::new_v4().to_string();
  let mut conn = pool.get().unwrap();

  let response = diesel::insert_into(users::table)
    .values(User { id: id.clone(), username: user.username })
    .get_result::<User>(&mut conn);

  if let Err(err) = response {
    return Err(ApiError::InternalServiceError(err.to_string()));
  }

  let result = users::table.find(id).first::<User>(&mut conn);

  match result {
    Ok(data) => Ok(ApiResponse::Created(Data { data, message: constants::CREATED.to_string() })),
    Err(err) => Err(ApiError::BadRequest(err.to_string())),
  }
}

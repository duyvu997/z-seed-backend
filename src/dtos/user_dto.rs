use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateRequest {
  pub username: String,
}

#[derive(ToSchema)]
pub struct UserDto {
  pub id: String,
  pub username: String,
}

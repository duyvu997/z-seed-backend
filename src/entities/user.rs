use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Default, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub username: String,
}

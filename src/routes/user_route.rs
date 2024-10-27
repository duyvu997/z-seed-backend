use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use sqlx::{Pool, Postgres};

use crate::services::user_service::{create, get_user};

pub fn route(pool: Arc<Pool<Postgres>>) -> Router {
  Router::new()
    .route("/:id", get(get_user))
    .route("/", post(create))
    .with_state(pool)
}

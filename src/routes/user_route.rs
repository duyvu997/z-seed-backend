use axum::{
  routing::{get, post},
  Router,
};
use diesel::{
  r2d2::{ConnectionManager, Pool},
  PgConnection,
};
use std::sync::Arc;

use crate::services::user_service::{create, get_by_id};

pub fn route(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Router {
  Router::new().route("/:id", get(get_by_id)).route("/", post(create)).with_state(pool)
}

use axum::{middleware::from_fn, routing::get, Router};
use dotenv::dotenv;
use std::sync::Arc;
use tracing::info;

use z_seed_backend::{configs::EnvDev, db, middlewares::error_handler, routes::user_route};

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();
  dotenv().ok();

  let cfg = match EnvDev::from_env() {
    Ok(cfg) => cfg,
    Err(_) => {
      info!("Loading Env Failed");
      return;
    },
  };

  // build our application with a route
  let pool = db::intialized_db(&cfg.web.dns, cfg.web.max_conns).await;

  let app = Router::new()
    // `GET /` goes to `root`‘
    .route("/", get(|| async { "Hello, World!" }))
    .nest("/api/users", user_route::route(Arc::new(pool)))
    .layer(from_fn(error_handler));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  info!("Server is running on port 3000");
  axum::serve(listener, app).await.unwrap();
}

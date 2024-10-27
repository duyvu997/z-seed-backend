use axum::{extract::Request, middleware::Next, response::Response};

pub async fn error_handler(req: Request, next: Next) -> Response{
  let response = next.run(req).await;

  response
}

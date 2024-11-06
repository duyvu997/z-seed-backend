pub mod api_doc;
pub mod common;
pub mod configs;
pub mod constants;
pub mod db;
pub mod schema;

pub mod middlewares;

pub mod dtos {
  pub mod user_dto;
}

pub mod routes {
  pub mod user_route;
}

pub mod services {
  pub mod user_service;
}

pub mod entities {
  pub mod user;
}

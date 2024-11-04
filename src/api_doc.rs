use utoipa::OpenApi;

use crate::{
  dtos::user_dto::UserDto,
  services::user_service::{__path_create, __path_get_by_id},
};

#[derive(OpenApi)]
#[openapi(paths(get_by_id, create), components(schemas(UserDto)))]
pub struct ApiDoc;

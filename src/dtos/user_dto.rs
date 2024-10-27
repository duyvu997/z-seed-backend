use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateRequest{
  pub username: String
}

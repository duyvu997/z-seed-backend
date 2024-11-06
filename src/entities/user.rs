use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: String,
  pub username: String,
}

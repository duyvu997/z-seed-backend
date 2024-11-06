use diesel::{
  r2d2::{ConnectionManager, Pool},
  PgConnection,
};

pub async fn intialized_db(dsn: &str, max_conns: u32) -> Pool<ConnectionManager<PgConnection>> {
  let manager = ConnectionManager::<PgConnection>::new(dsn);

  Pool::builder().max_size(max_conns).build(manager).unwrap()
}

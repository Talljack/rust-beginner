use serde::{ Deserialize, Serialize };
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub username: String,
}

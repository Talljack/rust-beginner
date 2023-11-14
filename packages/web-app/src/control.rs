use diesel::prelude::*;
use uuid::Uuid;
use crate::model;

type DbError = Box<dyn std::error::Error + Send + Sync>;


pub fn find_user_by_id(
  conn: &mut SqliteConnection,
  uid: Uuid,
) -> Result<Option<model::User>, DbError> {
  use crate::schema::users::dsl::*;
  let user = users
    .filter(id.eq(uid.to_string()))
    .first::<model::User>(conn)
    .optional()?;
  Ok(user)
}

pub fn insert_new_user(
  conn: &mut SqliteConnection,
  new_name: &str,
) -> Result<model::User, DbError> {
  use crate::schema::users::dsl::*;
  let new_user = model::User {
    id: Uuid::new_v4().to_string(),
    name: new_name.to_string(),
  };
  diesel::insert_into(users)
    .values(&new_user)
    .execute(conn)?;
  Ok(new_user)
}

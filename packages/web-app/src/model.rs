use serde::{Deserialize, Serialize};
use crate::schema::users::dsl::users;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewUser {
  pub name: String,
}

impl NewUser {
  #[cfg(test)]
  pub fn new(name: impl Into<String>) -> Self {
    Self { name: name.into() }
  }
}

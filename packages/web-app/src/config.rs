use serde::Deserialize;
use deadpool_postgres::{Config as PgConfig, Pool};

#[derive(Deserialize, Debug, Default)]
pub struct Config {
  pub server_addr: String,
  pub pg: PgConfig,
}

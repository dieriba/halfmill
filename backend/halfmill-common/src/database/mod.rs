mod user;

use crate::config::config;
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
pub use user::*;

type Db = PgPool;

#[derive(Clone)]
pub struct Database {
    connection_pool: Db,
}

impl Database {
    pub async fn connect() -> Result<Self> {
        tracing::info!("Acquiring pool of database connections...");
        let connection_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config().database_url)
            .await?;
        tracing::info!("Pool Acquired successfully!");
        Ok(Self { connection_pool })
    }

    pub(in crate::database) fn db(&self) -> &Db {
        &self.connection_pool
    }
}

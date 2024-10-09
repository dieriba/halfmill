mod user;

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::config;

type Db = PgPool;

pub struct Database {
    connection_pool: Db,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let connection_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config().database_url)
            .await?;
        Ok(Self { connection_pool })
    }

    pub(in crate::database) fn db(&self) -> &Db {
        &self.connection_pool
    }
}

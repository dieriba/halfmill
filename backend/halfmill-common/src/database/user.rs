use super::Database;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

pub struct UserAction;

impl UserAction {
    pub async fn create(database: &Database) -> Result<String> {
        let _ = database.db();
        Ok(String::new())
    }
}

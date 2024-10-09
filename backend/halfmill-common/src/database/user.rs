use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::Database;

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
        let pool = database.db();

        //        let query = sqlx::query_as::<_, String>("INSERT INTO user ");

        Ok(String::new())
    }
}

use std::{any::TypeId, marker::PhantomData};

use crate::HttpError;

use super::Database;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow,  FromRow, QueryBuilder};
use user_row::Deserializable;
use uuid::Uuid;
mod user_row {
    pub trait Deserializable {
        fn query(criteria: &str) -> String;
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserId {
    pub id: Uuid,
}

impl user_row::Deserializable for UserId {
    fn query(criteria: &str) -> String {
        format!("SELECT id FROM users WHERE {} = ", criteria)
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserIdWithPassword {
    pub id: Uuid,
    #[serde(skip_serializing)]
    pub password: String
}

impl user_row::Deserializable for UserIdWithPassword {
    fn query(criteria: &str) -> String {
        format!("SELECT id, password FROM users WHERE {} = ", criteria)
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub username: String,
}

impl user_row::Deserializable for User {
    fn query(criteria: &str) -> String {
        format!("SELECT username FROM users WHERE {} = ", criteria)
    }
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct UserWithPassword {
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl user_row::Deserializable for UserWithPassword {
    fn query(criteria: &str) -> String {
        format!("SELECT username , password FROM users WHERE {} = ", criteria)
    }
}

impl UserWithPassword {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub struct UserAction;

impl UserAction {
    pub async fn create(
        database: &Database,
        UserWithPassword { username, password }: UserWithPassword,
    ) -> Result<User, HttpError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password) values ($1, $2) returning username",
        )
        .bind(username)
        .bind(password)
        .fetch_one(database.db())
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(database) if database.is_unique_violation() => {
                HttpError::unprocessable_entity(Some("username already taken, please choose another".to_string()))
            }
            sqlx::Error::ColumnNotFound(column) => {
                tracing::info!("You probably give out the wrong column name to the query double check it please!, here was the the searched column: {}", column);
                HttpError::internal_server_error()
            }
            _ => HttpError::internal_server_error(),
        })
    }

    pub async fn get_by_username<'de, T>(database: &Database, username: &str) -> Result<T, HttpError>
    where
        T: Send
            + Unpin
            + for<'r> FromRow<'r, PgRow>
            + Deserialize<'de>
            + Serialize
            + 'static
            + user_row::Deserializable,
    {
        get_by(database, "username", username, PhantomData).await
    }

    pub async fn get_by_id<'de, T>(database: &Database, id: &str) -> Result<T, HttpError>
    where
        T: Send
            + Unpin
            + for<'r> FromRow<'r, PgRow>
            + Deserialize<'de>
            + Serialize
            + 'static
            + user_row::Deserializable,
    {
        get_by(database, "id", id, PhantomData).await
    }

    pub async fn check_if_already_exist_by_username(
        database: &Database,
        predicate_value: &[u8],
    ) -> Result<(), HttpError> {
        UserAction::already_exist(database, "username", predicate_value).await
    }

    pub async fn check_if_already_exist_by_id(
        database: &Database,
        predicate_value: &[u8],
    ) -> Result<(), HttpError> {
        UserAction::already_exist(database, "id", predicate_value).await
    }

    async fn already_exist(
        database: &Database,
        predicate_key: &str,
        predicate_value: &[u8],
    ) -> Result<(), HttpError> {
        let user = sqlx::query_as::<_, User>("SELECT username FROM users WHERE ($1)='($2)'")
            .bind(predicate_key)
            .bind(predicate_value)
            .fetch_one(database.db())
            .await;
        if let Err(sqlx::Error::Database(database)) = user {
            if database.is_unique_violation() {
                return Err(HttpError::unprocessable_entity(
                    Some("username is already taken please choose another one".to_string()),
                ));
            }
        }
        Ok(())
    }
}
async fn get_by<'de, T>(
    database: &Database,
    predicate_key: &str,
    predicate_value: &str,
    _marker: PhantomData<T>,
) -> Result<T, HttpError>
where
    T: Send
        + Unpin
        + for<'r> FromRow<'r, PgRow>
        + Deserialize<'de>
        + Serialize
        + 'static
        + user_row::Deserializable,
{
    let query = match TypeId::of::<T>() {
        id if id == TypeId::of::<User>() => User::query(predicate_key),
        id if id == TypeId::of::<UserWithPassword>() => UserWithPassword::query(predicate_key),
        id if id == TypeId::of::<UserIdWithPassword>() => UserIdWithPassword::query(predicate_key),
        _ => unreachable!(),
    };
    let mut query = QueryBuilder::new(query);
    query.push_bind(predicate_value);
    query.build_query_as()
        .bind(predicate_key)
        .bind(predicate_value)
        .fetch_one(database.db())
        .await
        .map_err(|err| {
            tracing::error!("{:#?}", err);
            match err {    
                sqlx::Error::RowNotFound => HttpError::resource_not_found(Some("User not found".to_string())),
                sqlx::Error::ColumnNotFound(column) => {
                    tracing::error!("You probably give out the wrong column name to the query or did not retrieve it double check it please!, here was the the searched column: {}", column);
                    HttpError::internal_server_error()
                }
            _ => HttpError::internal_server_error()
            }
        })
}

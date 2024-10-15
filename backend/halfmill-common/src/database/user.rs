use std::{any::TypeId, marker::PhantomData};

use crate::Error;

use super::Database;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};
use user_row::Deserializable;
mod user_row {
    pub trait Deserializable {
        fn query() -> &'static str;
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub username: String,
}

impl user_row::Deserializable for User {
    fn query() -> &'static str {
        "SELECT username FROM users WHERE ($1)='($2)'"
    }
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct UserWithPassword {
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl user_row::Deserializable for UserWithPassword {
    fn query() -> &'static str {
        "SELECT username, password FROM users WHERE ($1)='($2)'"
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
    ) -> Result<User, Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password) values ($1, $2) returning username",
        )
        .bind(username)
        .bind(password)
        .fetch_one(database.db())
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(database) if database.is_unique_violation() => {
                Error::AlreadyExists("username already taken, please choose another".to_string())
            }
            sqlx::Error::ColumnNotFound(column) => {
                tracing::info!("You probably give out the wrong column name to the query double check it please!, here was the the searched column: {}", column);
                Error::InternalErr("internal server error".to_string())
            }
            _ => Error::InternalErr(err.to_string()),
        })
    }

    pub async fn get_by_username<'de, T>(database: &Database, username: &[u8]) -> Result<T, Error>
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

    pub async fn get_by_id<'de, T>(database: &Database, id: &[u8]) -> Result<T, Error>
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
    ) -> Result<(), Error> {
        UserAction::already_exist(database, "username", predicate_value).await
    }

    pub async fn check_if_already_exist_by_id(
        database: &Database,
        predicate_value: &[u8],
    ) -> Result<(), Error> {
        UserAction::already_exist(database, "id", predicate_value).await
    }

    async fn already_exist(
        database: &Database,
        predicate_key: &str,
        predicate_value: &[u8],
    ) -> Result<(), Error> {
        let user = sqlx::query_as::<_, User>("SELECT username FROM users WHERE ($1)='($2)'")
            .bind(predicate_key)
            .bind(predicate_value)
            .fetch_one(database.db())
            .await;
        if let Err(sqlx::Error::Database(database)) = user {
            if database.is_unique_violation() {
                return Err(Error::AlreadyExists(
                    "username is already taken please choose another one".to_string(),
                ));
            }
        }
        Ok(())
    }
}
async fn get_by<'de, T>(
    database: &Database,
    predicate_key: &str,
    predicate_value: &[u8],
    _marker: PhantomData<T>,
) -> Result<T, Error>
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
        id if id == TypeId::of::<User>() => User::query(),
        id if id == TypeId::of::<UserWithPassword>() => UserWithPassword::query(),
        _ => unreachable!(),
    };

    sqlx::query_as::<_, T>(query)
        .bind(predicate_key)
        .bind(predicate_value)
        .fetch_one(database.db())
        .await
        .map_err(|err| {
            tracing::error!("{:#?}", err);
            match err {    
                sqlx::Error::RowNotFound => Error::NotFound("User not found".to_string()),
                sqlx::Error::ColumnNotFound(column) => {
                    tracing::info!("You probably give out the wrong column name to the query double check it please!, here was the the searched column: {}", column);
                    Error::InternalErr("internal server error".to_string())
                }
            _ => Error::InternalErr(err.to_string())
            }
        })
}

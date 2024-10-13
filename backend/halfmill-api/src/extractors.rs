use async_trait::async_trait;
use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;
        data.validate().map_err(|e| {
            let err = ServerError::ValidationError(e);
            err.into_response()
        })?;

        return Ok(ValidatedJson(data));
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(validator::ValidationErrors),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let message = match self {
            ServerError::ValidationError(validation_error) => {
                format!("{validation_error}").replace("\n", ",")
            }
        };

        (StatusCode::BAD_REQUEST, Json(ErrorResponse { message })).into_response()
    }
}

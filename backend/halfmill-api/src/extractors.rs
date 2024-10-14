use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
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
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|e| {
            let err = ServerError::AxumRejectionError(e);
            err.into_response()
        })?;
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
    #[error(transparent)]
    AxumRejectionError(JsonRejection),
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    message: Vec<String>,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let error_message = match self {
            ServerError::ValidationError(validation_error) => {
                let all_errors_message = validation_error
                    .field_errors()
                    .values()
                    .flat_map(|errors| {
                        errors
                            .iter()
                            .map(|err| err.message.clone().unwrap().into_owned())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<String>>();
                all_errors_message
            }
            Self::AxumRejectionError(err) => {
                let error_message = err.body_text();
                return (
                    err.status(),
                    Json(ErrorResponse {
                        message: vec![error_message],
                    }),
                )
                    .into_response();
            }
            _ => unreachable!(),
        };

        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: error_message,
            }),
        )
            .into_response()
    }
}

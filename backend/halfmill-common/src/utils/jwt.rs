use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{HttpError, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: Uuid, exp: usize) -> Self {
        Self { sub, exp }
    }
}
pub struct JWTManager {
    header: Header,
    encoded_access_token: EncodingKey,
    encoded_refresh_token: EncodingKey,
    decoding_access_token: DecodingKey,
    decoding_refresh_token: DecodingKey,
    validation_algorithm: Validation,
}

impl JWTManager {
    pub fn new(access_token_secret: &str, refresh_token_secret: &str) -> Self {
        let access_token_secret = access_token_secret.as_bytes();
        let refresh_token_secret = refresh_token_secret.as_bytes();
        Self {
            header: Header::default(),
            encoded_access_token: EncodingKey::from_secret(access_token_secret),
            encoded_refresh_token: EncodingKey::from_secret(refresh_token_secret),
            decoding_access_token: DecodingKey::from_secret(access_token_secret),
            decoding_refresh_token: DecodingKey::from_secret(refresh_token_secret),
            validation_algorithm: Validation::default(),
        }
    }

    pub fn get_access_token(&self, data: &Claims) -> Result<String, HttpError> {
        get_token(&self.header, data, &self.encoded_access_token)
    }

    pub fn get_refresh_token(&self, data: &Claims) -> Result<String, HttpError> {
        get_token(&self.header, data, &self.encoded_refresh_token)
    }

    pub fn validate_access_token(&self, token: &str) -> Result<UserId, HttpError> {
        validate_token(
            token,
            &self.decoding_access_token,
            &self.validation_algorithm,
        )
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<UserId, HttpError> {
        validate_token(
            token,
            &self.decoding_refresh_token,
            &self.validation_algorithm,
        )
    }

    pub fn get_current_timestamp() -> u64 {
        jsonwebtoken::get_current_timestamp()
    }
}

fn get_token(
    header: &Header,
    data: &Claims,
    encoding_key: &EncodingKey,
) -> Result<String, HttpError> {
    encode(header, &data, encoding_key).map_err(|_| HttpError::internal_server_error())
}

fn validate_token(
    token: &str,
    decoding_key: &DecodingKey,
    validation_algorithm: &Validation,
) -> Result<UserId, HttpError> {
    let TokenData { claims, .. } = decode::<Claims>(token, decoding_key, validation_algorithm)
        .map_err(|err| match err.into_kind() {
            ErrorKind::ExpiredSignature | ErrorKind::InvalidSignature => HttpError::unauthorized(),
            e => {
                tracing::error!("{:#?}", e);
                HttpError::internal_server_error()
            }
        })?;
    println!("{:#?}", claims);
    Ok(UserId::new(claims.sub))
}

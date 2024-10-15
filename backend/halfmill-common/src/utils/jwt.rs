use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};

use crate::{Error, UserId};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
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
            encoded_refresh_token: EncodingKey::from_secret(access_token_secret),
            decoding_access_token: DecodingKey::from_secret(access_token_secret),
            decoding_refresh_token: DecodingKey::from_secret(refresh_token_secret),
            validation_algorithm: Validation::new(Algorithm::HS256),
        }
    }

    pub fn get_access_token(&self, data: UserId) -> Result<String, Error> {
        get_token(&self.header, data, &self.encoded_access_token)
    }

    pub fn get_refresh_token(&self, data: UserId) -> Result<String, Error> {
        get_token(&self.header, data, &self.encoded_refresh_token)
    }

    pub fn validate_access_token(&self, token: &str) -> Result<UserId, Error> {
        validate_token(
            token,
            &self.decoding_access_token,
            &self.validation_algorithm,
        )
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<UserId, Error> {
        validate_token(
            token,
            &self.decoding_refresh_token,
            &self.validation_algorithm,
        )
    }
}

fn get_token(header: &Header, data: UserId, encoding_key: &EncodingKey) -> Result<String, Error> {
    encode(header, &data, encoding_key)
        .map_err(|_| Error::InternalErr("Internal Server Error".to_string()))
}

fn validate_token(
    token: &str,
    decoding_key: &DecodingKey,
    validation_algorithm: &Validation,
) -> Result<UserId, Error> {
    let TokenData { claims, .. } = decode::<UserId>(token, decoding_key, validation_algorithm)
        .map_err(|err| match err.into_kind() {
            ErrorKind::ExpiredSignature | ErrorKind::InvalidSignature => {
                Error::Unauthorized("Unauthorized".to_string())
            }
            e => {
                tracing::error!("{:#?}", e);
                Error::InternalErr("Internal Server Error".to_string())
            }
        })?;
    Ok(claims)
}

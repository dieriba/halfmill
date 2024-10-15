use argon2::{
    password_hash,
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::{constant_message::WRONG_CREDENTIALS, HttpError};

#[derive(Debug, Default)]
pub struct PasswordManager<'key>(Argon2<'key>);

impl<'key> PasswordManager<'key> {
    pub fn hash_password(&self, password: &[u8]) -> Result<String, HttpError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .0
            .hash_password(password, &salt)
            .map_err(|_| HttpError::internal_server_error())?
            .to_string();
        Ok(password_hash)
    }

    pub fn compare_password(
        &self,
        password: &[u8],
        hashed_password: &str,
    ) -> Result<(), HttpError> {
        let parsed_hash =
            PasswordHash::new(hashed_password).map_err(|_| HttpError::internal_server_error())?;
        self.0
            .verify_password(password, &parsed_hash)
            .map_err(|err| {
                if let password_hash::Error::Password = err {
                    return HttpError::bad_request(Some(WRONG_CREDENTIALS.to_string()));
                }

                HttpError::internal_server_error()
            })
    }
}

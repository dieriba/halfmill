use argon2::{
    password_hash,
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::Error;
#[derive(Debug, Default)]
pub struct PasswordManager<'key>(Argon2<'key>);

impl<'key> PasswordManager<'key> {
    pub fn hash_password(&self, password: &[u8]) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .0
            .hash_password(password, &salt)
            .map_err(|err| Error::InternalErr(err.to_string()))?
            .to_string();
        Ok(password_hash)
    }

    pub fn compare_password(&self, password: &[u8], hashed_password: &str) -> Result<(), Error> {
        let parsed_hash =
            PasswordHash::new(hashed_password).map_err(|e| Error::InternalErr(e.to_string()))?;
        self.0
            .verify_password(password, &parsed_hash)
            .map_err(|err| {
                if let password_hash::Error::Password = err {
                    return Error::BadRequest("Wrong Credentials".to_string());
                }

                Error::InternalErr(err.to_string())
            })
    }
}

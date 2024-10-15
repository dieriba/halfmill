use super::constant::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Validate, Deserialize)]
pub struct CreateUserDto {
    #[validate(length(
        min = USERNAME_MIN_LENGTH,
        max = USERNAME_MAX_LENGTH,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = PASSWORD_MIN_LENGTH,
        max = PASSWORD_MAX_LENGTH,
        message = "Password must be between 8 and 50 characters"
    ))]
    pub password: String,
    #[serde(rename(deserialize = "confirmPassword"))]
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    confirm_password: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct LoginUserDto {
    #[validate(length(
        min = USERNAME_MIN_LENGTH,
        message = "username must not be empty"
    ))]
    pub username: String,
    #[validate(length(
        min = PASSWORD_MIN_LENGTH,
        message = "password must be not empty"
    ))]
    pub password: String,
}

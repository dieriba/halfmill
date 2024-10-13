use super::constant::*;
use serde::Deserialize;
use validator::Validate;
#[derive(Validate, Deserialize)]
pub struct CreateUser {
    #[validate(length(
        min = USERNAME_MIN_LENGTH,
        max = USERNAME_MAX_LENGTH,
        message = "Username must be between 3 and 20 characters"
    ))]
    username: String,
    #[validate(length(
        min = PASSWORD_MIN_LENGTH,
        max = PASSWORD_MAX_LENGTH,
        message = "Password must be between 8 and 50 characters"
    ))]
    password: String,
    #[serde(rename(deserialize = "confirmPassword"))]
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    confirm_password: String,
}

use std::borrow::Cow;

use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Validate, Deserialize)]
pub struct ScriptDto {
    #[validate(custom(function = "is_supported_language"))]
    pub language: String,
    #[validate(length(min = 1, message = "script should not be empty"))]
    pub content: String,
}

fn is_supported_language(language: &str) -> Result<(), ValidationError> {
    match language {
        "python" | "typescript" => Ok(()),
        _ => {
            let validation_error = ValidationError::new("");
            let message = Cow::from(format!("`{}` language is not supported, only `python` and `typescript` are supported language", language));
            Err(validation_error.with_message(message))
        }
    }
}

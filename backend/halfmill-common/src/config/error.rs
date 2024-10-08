use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The following env variable is missing: {0}")]
    MissingEnvVariable(&'static str),
}

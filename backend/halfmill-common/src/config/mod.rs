mod error;

use anyhow::Result;
use std::env;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    CONFIG.get_or_init(|| {
        Config::new().unwrap_or_else(|err| {
            tracing::error!("{err}");
            panic!();
        })
    })
}

pub struct Config {
    pub database_url: String,
    pub backend_port: String,
    pub access_token_secret: String,
    pub refresh_token_secret: String,
}

impl Config {
    fn new() -> Result<Self> {
        Ok(Self {
            database_url: get_env("DATABASE_URL")?,
            backend_port: get_env("BACKEND_PORT")?,
            access_token_secret: get_env("ACCESS_TOKEN_SECRET")?,
            refresh_token_secret: get_env("REFRESH_TOKEN_SECRET")?,
        })
    }
}

fn get_env(variable: &'static str) -> Result<String> {
    Ok(env::var(variable).map_err(|_| error::Error::MissingEnvVariable(variable))?)
}

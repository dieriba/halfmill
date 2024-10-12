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
}

impl Config {
    pub fn new() -> Result<Self> {
        Ok(Self {
            database_url: get_env("DATABASE_URL")?,
            backend_port: get_env("BACKEND_PORT")?,
        })
    }
}

fn get_env(variable: &'static str) -> Result<String> {
    Ok(env::var(variable).map_err(|_| error::Error::MissingEnvVariable(variable))?)
}

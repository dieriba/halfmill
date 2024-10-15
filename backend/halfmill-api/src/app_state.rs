use std::sync::Arc;

use axum::extract::FromRef;
use halfmill_common::{
    config::{config, Config},
    Database, JWTManager, PasswordManager,
};

#[derive(Clone)]
pub struct AppState<'key> {
    pub database: Database,
    pub password_manager: Arc<PasswordManager<'key>>,
    pub jwt_manager: Arc<JWTManager>,
}
#[derive(Clone)]
pub struct AppStateWrapper<'key>(pub Arc<AppState<'key>>);

impl<'key> FromRef<AppStateWrapper<'key>> for Database {
    fn from_ref(app_state: &AppStateWrapper) -> Database {
        app_state.0.database.clone()
    }
}

impl<'key> FromRef<AppStateWrapper<'key>> for Arc<JWTManager> {
    fn from_ref(app_state: &AppStateWrapper) -> Arc<JWTManager> {
        app_state.0.jwt_manager.clone()
    }
}

pub fn init_app_state<'key>(database: Database) -> AppStateWrapper<'key> {
    let Config {
        access_token_secret,
        refresh_token_secret,
        ..
    } = config();
    AppStateWrapper(Arc::new(AppState {
        database,
        password_manager: Arc::new(PasswordManager::default()),
        jwt_manager: Arc::new(JWTManager::new(access_token_secret, refresh_token_secret)),
    }))
}

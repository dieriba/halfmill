use std::sync::Arc;

use axum::extract::FromRef;
use halfmill_common::{Database, PasswordManager};

#[derive(Clone)]
pub struct AppState<'key> {
    pub database: Database,
    pub password_manager: Arc<PasswordManager<'key>>,
}
#[derive(Clone)]
pub struct AppStateWrapper<'key>(pub Arc<AppState<'key>>);

impl<'key> FromRef<AppStateWrapper<'key>> for Database {
    fn from_ref(app_state: &AppStateWrapper) -> Database {
        app_state.0.database.clone()
    }
}

pub fn init_app_state<'key>(database: Database) -> AppStateWrapper<'key> {
    AppStateWrapper(Arc::new(AppState {
        database,
        password_manager: Arc::new(PasswordManager::default()),
    }))
}

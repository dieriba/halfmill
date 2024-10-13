use std::sync::Arc;

use axum::extract::FromRef;
use halfmill_common::Database;

#[derive(Clone)]
struct AppState {
    pub database: Database,
}
#[derive(Clone)]
pub struct AppStateWrapper(Arc<AppState>);

impl FromRef<AppStateWrapper> for Database {
    fn from_ref(app_state: &AppStateWrapper) -> Database {
        app_state.0.database.clone()
    }
}

pub fn init_app_state(database: Database) -> AppStateWrapper {
    AppStateWrapper(Arc::new(AppState { database }))
}

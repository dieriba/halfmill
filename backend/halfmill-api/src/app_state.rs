use std::sync::Arc;

use axum::extract::FromRef;
use halfmill_common::Database;

#[derive(Clone)]
pub struct AppState {
    database: Database,
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Database {
        app_state.database.clone()
    }
}

pub fn init_app_state(database: Database) -> Arc<AppState> {
    Arc::new(AppState { database })
}

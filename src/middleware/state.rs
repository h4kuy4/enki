use std::sync::Arc;

use axum::Extension;
use sea_orm::DatabaseConnection;

pub struct State {
    pub conn: DatabaseConnection,
}

impl State {
    pub fn new(conn: DatabaseConnection) -> Extension<Arc<State>> {
        Extension(Arc::new(State { conn }))
    }
}

use axum::{Extension, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;

use std::sync::Arc;

use crate::middleware::State;

mod api;

pub fn init(conn: DatabaseConnection) -> Router {
    Router::new()
        .nest("/api", api::init())
        .layer(Extension(Arc::new(State { conn })))
        .layer(CorsLayer::permissive())
}

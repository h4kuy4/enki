use axum::{Extension, Router};
use jwt_auth::Jwt;
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;

use std::sync::Arc;

use crate::{middleware::State, model::Account};

mod api;

pub fn init(conn: DatabaseConnection, jwt: Jwt, account: Account) -> Router {
    Router::new()
        .nest("/api", api::init())
        .layer(Extension(Arc::new(State { conn, jwt, account })))
        .layer(CorsLayer::permissive())
}

use axum::Router;

pub mod auth;
pub mod category;
pub mod friend;
pub mod manage;
pub mod post;

pub fn init() -> Router {
    Router::new().nest("/v1", v1::init())
}

pub mod v1 {
    use axum::{routing::get, Router};

    use super::{auth, category, manage, post};

    pub fn init() -> Router {
        Router::new()
            .route("/ping", get(|| async { "pong!" }))
            .nest("/post", post::v1::init())
            .nest("/manage", manage::v1::init())
            .nest("/category", category::v1::init())
            .nest("/auth", auth::v1::init())
            .nest("/friend", super::friend::v1::init())
        // .nest("/comment", comment::init())
    }
}

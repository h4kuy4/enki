use axum::Router;

pub mod category;
pub mod manage;
pub mod post;
pub mod tag;

pub fn init() -> Router {
    Router::new().nest("/v1", v1::init())
}

pub mod v1 {
    use axum::{routing::get, Router};

    use super::{category, manage, post, tag};

    pub fn init() -> Router {
        Router::new()
            .route("/ping", get(|| async { "pong!" }))
            .nest("/post", post::v1::init())
            .nest("/manage", manage::v1::init())
            .nest("/tag", tag::v1::init())
            .nest("/category", category::v1::init())
        // .nest("/auth", auth::init())
        // .nest("/comment", comment::init())
        // .nest("/manage", manage::init())
    }
}

pub mod v1 {
    use axum::{routing::post, Router};

    use crate::service;

    pub fn init() -> Router {
        Router::new().route("/login", post(service::auth::login))
    }
}

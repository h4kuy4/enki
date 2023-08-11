pub mod v1 {
    use axum::{routing::get, Router};

    use crate::service;

    pub fn init() -> Router {
        Router::new().route("/", get(service::friend::list))
    }
}

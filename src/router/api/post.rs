pub mod v1 {
    use axum::{
        routing::{delete, get, patch, post},
        Router,
    };

    use crate::service;

    pub fn init() -> Router {
        Router::new()
            .route("/", get(service::post::list))
            .route("/:id", get(service::post::get))
    }
}

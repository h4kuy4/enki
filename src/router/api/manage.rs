pub mod v1 {
    use axum::Router;
    pub fn init() -> Router {
        Router::new()
            .nest("/post", post::init())
            .nest("/tag", tag::init())
    }

    pub mod post {
        use axum::{
            routing::{delete, get, patch, post},
            Router,
        };

        use crate::service;

        pub fn init() -> Router {
            Router::new()
                .route("/", get(service::manage::post::list))
                .route("/", post(service::manage::post::add))
                .route("/:id", get(service::manage::post::get))
                .route("/:id", patch(service::manage::post::update))
                .route("/:id", delete(service::manage::post::delete))
        }
    }

    pub mod tag {
        use axum::{
            routing::{delete, get, patch, post},
            Router,
        };

        use crate::service;

        pub fn init() -> Router {
            Router::new()
                .route("/", get(service::manage::tag::list))
                .route("/", post(service::manage::tag::add))
                .route("/:id", get(service::manage::tag::get))
                .route("/:id", patch(service::manage::tag::update))
                .route("/:id", delete(service::manage::tag::delete))
        }
    }
}

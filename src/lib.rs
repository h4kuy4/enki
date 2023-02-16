pub mod config;
pub mod database;
pub mod deserializer;
pub mod entity;
pub mod error;
pub mod middleware;
pub mod model;
pub mod response;
pub mod router;
pub mod serializer;
pub mod service;
pub mod utils;

pub use crate::config::Config;
pub use error::Error;
pub use error::ErrorType;
pub use response::Response;

pub type Result<T> = std::result::Result<T, Error>;

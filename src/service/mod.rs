use sea_orm::DatabaseConnection;

use crate::middleware::state::State;

pub mod auth;
pub mod category;
pub mod friend;
pub mod manage;
pub mod post;

pub fn get_conn<'a>(state: &'a State) -> &'a DatabaseConnection {
    &state.conn
}

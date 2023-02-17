use sea_orm::DatabaseConnection;

use crate::middleware::state::State;

pub mod manage;
pub mod post;
pub mod tag;

pub fn get_conn<'a>(state: &'a State) -> &'a DatabaseConnection {
    &state.conn
}
